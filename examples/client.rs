use anyhow::Result;
use grpc_chat::client::Client;
use std::env;
use tokio::io::{self, AsyncBufReadExt};
//추적 가입자를 구현하고 구성하기 위한 유틸리티

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    //username
    //sender
    let username = "sk";
    // let username = env::var("USERNAME")?;
    println!("username{}", username);
    let mut client = Client::new(username).await;
    client.login().await?;
    client.get_messages().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    while let Ok(Some(line)) = stdin.next_line().await {
        client.send_message("lobby", username, line).await?;
    }

    Ok(())
}
