use ai_review::mcp::run_mcp_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_mcp_server().await
}
