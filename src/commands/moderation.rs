use log::error;
use serenity::model::id::UserId;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::{Channel, Message},
    prelude::*,
};

#[command]
#[required_permissions("MANAGE_CHANNELS")]
async fn slow_mode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u64>() {
        if let Err(why) = msg
            .channel_id
            .edit(&ctx.http, |c| c.slow_mode_rate(slow_mode_rate_seconds))
            .await
        {
            error!("Error setting channel's slow mode rate: {:?}", why);

            format!(
                "Failed to set slow mode to `{}` seconds.",
                slow_mode_rate_seconds
            )
        } else {
            format!(
                "Successfully set slow mode rate to `{}` seconds.",
                slow_mode_rate_seconds
            )
        }
    } else if let Some(Channel::Guild(channel)) = msg.channel_id.to_channel_cached(&ctx.cache).await
    {
        format!(
            "Current slow mode rate is `{}` seconds.",
            channel.slow_mode_rate.unwrap_or(0)
        )
    } else {
        "Failed to find channel in cache.".to_string()
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, say_content).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let user = args.parse::<UserId>().unwrap();

    if user.0 == msg.author.id.0 {
        if let Err(why) = msg.reply(&ctx.http, "you cannot ban yourself.").await {
            println!("Error sending message: {:?}", why);
        }
        return Ok(());
    }

    if let Err(why) = guild.ban(&ctx.http, &user, 0).await {
        error!("Could not ban user from guild: {}", why);
    } else {
        msg.reply(
            &ctx.http,
            format!(
                "user {} was banned from this guild by {}",
                &user.mention(),
                msg.author.mention()
            ),
        )
        .await?;
    }

    Ok(())
}
