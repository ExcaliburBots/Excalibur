use serenity::{
    framework::standard::{
        CommandResult, Args,
        macros::command,
    },
    model::channel::Message,
    prelude::*,
    utils::{content_safe, ContentSafeOptions}
};

use super::checks::*;

#[command]
// Limit command usage to guilds.
#[only_in(guilds)]
#[checks(GuildOwner)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! : )").await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let settings = if let Some(guild_id) = msg.guild_id {
        ContentSafeOptions::default()
            .clean_channel(false)
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let content = content_safe(&ctx.cache, &args.rest(), &settings).await;

    if let Err(why) = msg.channel_id.say(&ctx.http, &content).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "This is a small test-bot! : )").await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}