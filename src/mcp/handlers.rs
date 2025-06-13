use anyhow::Result;
use rmcp::{Error as McpError, model::*};
use std::process::Command;
use std::fs;

use super::types::{PopupRequest, McpResponseContent};

pub fn create_tauri_popup(request: &PopupRequest) -> Result<String> {
    // 创建临时请求文件 - 跨平台适配
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("mcp_request_{}.json", request.id));
    let request_json = serde_json::to_string_pretty(request)?;
    fs::write(&temp_file, request_json)?;

    // 调用全局安装的等一下命令（弹窗UI）
    let output = Command::new("等一下")
        .arg("--mcp-request")
        .arg(&temp_file.to_string_lossy().to_string())
        .output()?;

    // 清理临时文件
    let _ = fs::remove_file(&temp_file);

    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout);
        let response = response.trim();
        if response.is_empty() {
            Ok("用户取消了操作".to_string())
        } else {
            Ok(response.to_string())
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("UI进程失败: {}", error);
    }
}

pub fn parse_mcp_response(response: &str) -> Result<Vec<Content>, McpError> {
    if response.trim() == "CANCELLED" || response.trim() == "用户取消了操作" {
        return Ok(vec![Content::text("用户取消了操作".to_string())]);
    }

    // 尝试解析为JSON数组（MCP响应格式）
    match serde_json::from_str::<Vec<McpResponseContent>>(response) {
        Ok(content_array) => {
            let mut result = Vec::new();
            let mut image_count = 0;

            // 分别收集用户文本和图片信息
            let mut user_text_parts = Vec::new();
            let mut image_info_parts = Vec::new();

            for content in content_array {
                match content.content_type.as_str() {
                    "text" => {
                        if let Some(text) = content.text {
                            user_text_parts.push(text);
                        }
                    }
                    "image" => {
                        if let Some(source) = content.source {
                            if source.source_type == "base64" {
                                image_count += 1;

                                // 先添加图片到结果中（图片在前）
                                result.push(Content::image(source.data.clone(), source.media_type.clone()));

                                // 添加图片信息到图片信息部分
                                let base64_len = source.data.len();
                                let preview = if base64_len > 50 {
                                    format!("{}...", &source.data[..50])
                                } else {
                                    source.data.clone()
                                };

                                // 计算图片大小（base64解码后的大小）
                                let estimated_size = (base64_len * 3) / 4; // base64编码后大约增加33%
                                let size_str = if estimated_size < 1024 {
                                    format!("{} B", estimated_size)
                                } else if estimated_size < 1024 * 1024 {
                                    format!("{:.1} KB", estimated_size as f64 / 1024.0)
                                } else {
                                    format!("{:.1} MB", estimated_size as f64 / (1024.0 * 1024.0))
                                };

                                let image_info = format!(
                                    "=== 图片 {} ===\n类型: {}\n大小: {}\nBase64 预览: {}\n完整 Base64 长度: {} 字符",
                                    image_count, source.media_type, size_str, preview, base64_len
                                );
                                image_info_parts.push(image_info);
                            }
                        }
                    }
                    _ => {
                        // 未知类型，作为文本处理
                        if let Some(text) = content.text {
                            user_text_parts.push(text);
                        }
                    }
                }
            }

            // 构建文本内容：用户文本 + 图片信息 + 注意事项
            let mut all_text_parts = Vec::new();

            // 1. 用户输入的文本
            if !user_text_parts.is_empty() {
                all_text_parts.extend(user_text_parts);
            }

            // 2. 图片详细信息
            if !image_info_parts.is_empty() {
                all_text_parts.extend(image_info_parts);
            }

            // 3. 兼容性说明
            if image_count > 0 {
                all_text_parts.push(format!(
                    "💡 注意：用户提供了 {} 张图片。如果 AI 助手无法显示图片，图片数据已包含在上述 Base64 信息中。",
                    image_count
                ));
            }

            // 将所有文本内容合并并添加到结果末尾（图片后面）
            if !all_text_parts.is_empty() {
                let combined_text = all_text_parts.join("\n\n");
                result.push(Content::text(combined_text));
            }

            if result.is_empty() {
                result.push(Content::text("用户未提供任何内容".to_string()));
            }

            Ok(result)
        }
        Err(_) => {
            // 如果不是JSON格式，作为纯文本处理
            Ok(vec![Content::text(response.to_string())])
        }
    }
}
