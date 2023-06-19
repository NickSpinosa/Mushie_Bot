mod bot;
mod datastructures;
mod error;
mod prelude;
mod secrets;

use crate::prelude::*;
use secrets::get_discord_token;

fn main() -> Result<()> {
    println!("Hello, world!");

    // adds comment to trigger test run maybe?
    println!("discord token: {}", get_discord_token()?);

    Ok(())
}
