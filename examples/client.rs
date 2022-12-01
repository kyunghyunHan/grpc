use anyhow::Result;
use grpc_chat::client::Client;
use std::env;
use tokio::io;
use tokio::io::AsyncBufReadExt;
#[tokio::main]
async fn main() -> Result<()> {
    let username = env::var("USERNAME")?;
    let mut client = Client::new(username).await;

    client.login().await?;
    client.get_messages().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    while let Ok(Some(line)) = stdin.next_line().await {
        let (room, content) = line.split_at(line.find(':').unwrap());
        client.send_message("room", line).await?;
    }
    Ok(())
}
