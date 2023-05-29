mod secrets;
mod error;
mod prelude;
mod bot;

use secrets::get_discord_token;
use crate::prelude::*;

fn main() -> Result<()>{
    println!("Hello, world!");

    println!("discord token: {}", get_discord_token()?);

    Ok(())
}
