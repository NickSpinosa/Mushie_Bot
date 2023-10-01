mod bot;
mod error;
mod prelude;
mod secrets;

use crate::{bot::config::build_client, prelude::*};
use std::{env, println};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    if let Some(token) = args.get(1) {
        let mut client = build_client(&token).await;

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }

    Ok(())
}
