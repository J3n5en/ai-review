// 使用新的模块化MCP实现
use ai_review::mcp::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_server().await
}
