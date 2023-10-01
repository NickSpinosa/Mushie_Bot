use crate::bot::commands::GENERAL_GROUP;
use serenity::client::Context;

use serenity::framework::StandardFramework;
use serenity::{
    async_trait,
    client::{Client, EventHandler},
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
};
use songbird::SerenityInit;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

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
    GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
}

fn build_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP)
}
