use shaco::ingame;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
    let client = ingame::InGameClient::new()?;

    let data = client.all_game_data(None).await?;

    println!("{:#?}", data);

    Ok(())
}
