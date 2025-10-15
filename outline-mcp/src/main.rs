mod config;
mod server;

use anyhow::Result;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stderr) // Log to stderr, not stdout (stdout is for MCP protocol)
        .init();

    tracing::info!("Starting Outline MCP server...");

    // Create and run the server
    server::run().await
}
