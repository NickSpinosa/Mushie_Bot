use serenity::client::Context;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
    Result as SerenityResult,
};
use songbird::SerenityInit;

#[group]
//#[commands(deafen, join, leave, mute, play, ping, undeafen, unmute)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn build_client(token: &str) -> Client {
    Client::builder(token, build_intents())
        .event_handler(Handler)
        .framework(build_framework())
        .register_songbird()
        .await
        .expect("Err creating client")
}

fn build_intents() -> GatewayIntents {
    GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT
}

fn build_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP)
}
