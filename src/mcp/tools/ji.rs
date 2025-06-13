use anyhow::Result;
use rmcp::{Error as McpError, model::*, tool};

use crate::memory::{MemoryManager, MemoryCategory};
use crate::mcp::JiyiRequest;

/// 全局记忆管理工具
///
/// 用于存储和管理重要的开发规范、用户偏好和最佳实践
#[derive(Clone)]
pub struct JiTool;

#[tool(tool_box)]
impl JiTool {
    #[tool(description = "ji 全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践")]
    pub async fn ji(
        #[tool(aggr)] request: JiyiRequest,
    ) -> Result<CallToolResult, McpError> {
        // 检查项目路径是否存在
        if !std::path::Path::new(&request.project_path).exists() {
            return Err(McpError::invalid_params(
                format!("项目路径不存在: {}", request.project_path),
                None
            ));
        }

        let manager = MemoryManager::new(&request.project_path)
            .map_err(|e| McpError::internal_error(format!("创建记忆管理器失败: {}", e), None))?;

        let result = match request.action.as_str() {
            "记忆" => {
                if request.content.trim().is_empty() {
                    return Err(McpError::invalid_params("缺少记忆内容".to_string(), None));
                }

                let category = match request.category.as_str() {
                    "rule" => MemoryCategory::Rule,
                    "preference" => MemoryCategory::Preference,
                    "pattern" => MemoryCategory::Pattern,
                    "context" => MemoryCategory::Context,
                    _ => MemoryCategory::Context,
                };

                let id = manager.add_memory(&request.content, category)
                    .map_err(|e| McpError::internal_error(format!("添加记忆失败: {}", e), None))?;

                format!("✅ 记忆已添加，ID: {}\n📝 内容: {}\n📂 分类: {:?}", id, request.content, category)
            }
            "回忆" => {
                manager.get_project_info()
                    .map_err(|e| McpError::internal_error(format!("获取项目信息失败: {}", e), None))?
            }
            _ => {
                return Err(McpError::invalid_params(
                    format!("未知的操作类型: {}", request.action),
                    None
                ));
            }
        };

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}
