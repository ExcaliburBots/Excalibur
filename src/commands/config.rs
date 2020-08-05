extern crate log;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Colour, MessageBuilder};

use crate::managers::{Database, DefaultPrefix};
use crate::models::guild_config::GuildConfig;
use crate::utils::{get_default_prefix, save_prefix};

#[command]
#[required_permissions("ADMINISTRATOR")]
#[sub_commands(prefix)]
#[description = "Config related commands, e.g. for changing the prefix."]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<Database>().unwrap();
    let default_prefix = data.get::<DefaultPrefix>().unwrap();

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_config = GuildConfig::get(i64::from(guild.id), default_prefix, pool)
        .await
        .unwrap();

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{} - Guild Config", guild.name));
                e.color(Colour::DARK_GREEN);

                e.field("ID", guild.id, true);
                e.field("Prefix", guild_config.prefix, true);

                e
            });

            m
        })
        .await
    {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<Database>().unwrap();

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_config = GuildConfig::get(
        i64::from(guild.id),
        get_default_prefix(ctx).await.as_str(),
        pool,
    )
    .await
    .unwrap();

    if args.len() > 0 {
        let new_prefix = args.single::<String>().unwrap();
        let response = MessageBuilder::new()
            .push("My new prefix is: `")
            .push(&new_prefix)
            .push("`.")
            .build();

        save_prefix(new_prefix, guild.id, ctx).await;

        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    } else {
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                format!("My prefix for this guild is: `{}`", guild_config.prefix),
            )
            .await
        {
            error!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}
