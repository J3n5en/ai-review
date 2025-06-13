use anyhow::Result;
use rmcp::{Error as McpError, model::*, tool};
use uuid::Uuid;

use crate::mcp::{ZhiRequest, PopupRequest, create_tauri_popup, parse_mcp_response};

/// 智能代码审查交互工具
///
/// 支持预定义选项、自由文本输入和图片上传
#[derive(Clone)]
pub struct ZhiTool;

#[tool(tool_box)]
impl ZhiTool {
    #[tool(description = "zhi 智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传")]
    pub async fn zhi(
        #[tool(aggr)] request: ZhiRequest,
    ) -> Result<CallToolResult, McpError> {
        let popup_request = PopupRequest {
            id: Uuid::new_v4().to_string(),
            message: request.message,
            predefined_options: if request.predefined_options.is_empty() {
                None
            } else {
                Some(request.predefined_options)
            },
            is_markdown: request.is_markdown,
        };

        match create_tauri_popup(&popup_request) {
            Ok(response) => {
                // 解析响应内容，支持文本和图片
                let content = parse_mcp_response(&response)?;
                Ok(CallToolResult::success(content))
            }
            Err(e) => {
                Err(McpError::internal_error(format!("弹窗创建失败: {}", e), None))
            }
        }
    }
}
