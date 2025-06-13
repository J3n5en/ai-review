use anyhow::Result;
use rmcp::{
    Error as McpError, ServerHandler, ServiceExt, RoleServer,
    model::*,
    transport::stdio,
    service::RequestContext,
    tool,
};

use super::tools::{ZhiTool, JiTool};

#[derive(Clone)]
pub struct ZhiServer {
    // 可以添加状态字段
}

impl ZhiServer {
    pub fn new() -> Self {
        Self {}
    }
}

#[tool(tool_box)]
impl ServerHandler for ZhiServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Zhi-mcp".to_string(),
                version: "0.1.0".to_string(),
            },
            instructions: Some("Zhi 智能代码审查工具，支持交互式对话和记忆管理".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ServerInfo, McpError> {
        Ok(self.get_info())
    }
}

// 将工具方法委托给专门的工具结构体
#[tool(tool_box)]
impl ZhiServer {
    #[tool(description = "zhi 智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传")]
    async fn zhi(
        &self,
        #[tool(aggr)] request: super::ZhiRequest,
    ) -> Result<CallToolResult, McpError> {
        ZhiTool::zhi(request).await
    }

    #[tool(description = "ji 全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践")]
    async fn ji(
        &self,
        #[tool(aggr)] request: super::JiyiRequest,
    ) -> Result<CallToolResult, McpError> {
        JiTool::ji(request).await
    }
}

/// 启动MCP服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // 创建并运行服务器
    let service = ZhiServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            eprintln!("启动服务器失败: {}", e);
        })?;

    // 等待服务器关闭
    service.waiting().await?;
    Ok(())
}
