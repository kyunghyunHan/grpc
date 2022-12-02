use anyhow::Result;
use grpc_chat::server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    server::start().await;
    Ok(())
}
