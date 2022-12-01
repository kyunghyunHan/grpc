use anyhow::Result;
use grpc_chat::server;

#[tokio::main]

async fn main() -> Result<()> {
    server::start().await;
    Ok(())
}
