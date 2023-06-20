use std::sync::Arc;

use serenity::Result as SerenityResult;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::Message,
    prelude::Context,
};
use songbird::{error::JoinError, Call};
use tokio::sync::Mutex;

#[group]
#[commands(join, play, leave, ping)]
struct General;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let _handler = join_channel(ctx, msg).await;

    Ok(())
}

// pulled out into a helper function so we can make use of the handler returned in the play command
async fn join_channel(ctx: &Context, msg: &Message) -> Result<Arc<Mutex<Call>>, JoinError> {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Err(JoinError::IllegalChannel);
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handler, result) = manager.join(guild_id, connect_to).await;

    return result.map(|_| handler);
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
async fn ping(context: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, "Pong!").await);

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a valid URL")
                .await,
        );

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        play_audio(&url, msg, ctx, handler_lock).await?;
    } else {
        if let Ok(lock) = join_channel(ctx, msg).await {
            play_audio(&url, msg, ctx, lock).await?;
        }
    }

    Ok(())
}

async fn play_audio(
    url: &String,
    msg: &Message,
    ctx: &Context,
    handler_lock: Arc<Mutex<Call>>,
) -> CommandResult {
    let mut handler = handler_lock.lock().await;

    let source = match songbird::ytdl(&url).await {
        Ok(source) => source,
        Err(why) => {
            println!("Err starting source: {:?}", why);

            check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

            return Ok(());
        }
    };

    handler.play_source(source);

    check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);

    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
