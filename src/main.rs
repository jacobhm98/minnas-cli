use anyhow::Result;
use rmcp::ServiceExt;
use tokio::io::{stdin, stdout};
use tracing::error;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    info!("Starting Minnas MCP Server");

    if let Err(e) = run_server().await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }

    info!("Server shutdown complete");
    Ok(())
}

async fn run_server() -> Result<()> {
    info!("Initializing RustFS MCP Server");

    let server = RustfsMcpServer::new().await?;

    info!("Starting MCP server with stdio transport");

    server
        .serve((stdin(), stdout()))
        .await
        .context("Failed to serve MCP server")?
        .waiting()
        .await
        .context("Error while waiting for server shutdown")?;

    Ok(())
}
