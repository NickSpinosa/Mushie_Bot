mod bot;
mod datastructures;
mod error;
mod prelude;
mod secrets;

use crate::{prelude::*, bot::config::build_client};
use secrets::get_discord_token;
use songbird::EventHandler;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    // adds comment to trigger test run maybe?
    println!("discord token: {}", get_discord_token()?);

    let token = get_discord_token()?;

    let mut client = build_client(&token).await;

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}
