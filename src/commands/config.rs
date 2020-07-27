use crate::managers::Database;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Colour;
extern crate log;

use crate::models::guild_config::GuildConfig;

#[command]
async fn config(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data.get::<Database>().unwrap();

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let mut guild_config = GuildConfig::get(i64::from(guild.id), "!", pool)
        .await
        .unwrap();

    if guild_config.guild_id == 696017449069445152 {
        guild_config.set_prefix("??", pool).await.unwrap();
    }

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
