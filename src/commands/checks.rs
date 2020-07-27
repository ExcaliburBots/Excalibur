use crate::managers::Database;
use serenity::{
    framework::standard::{macros::check, Args, CheckResult, CommandOptions},
    model::{channel::Message, id::UserId},
    prelude::*,
};

#[check]
#[name = "GuildOwner"]
#[check_in_help]
pub async fn guild_owner_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    let guild = msg.guild(&ctx.cache).await;
    let owner_id = match &guild {
        Some(x) => x.owner_id,
        None => UserId(0),
    };
    if owner_id != 0 {
        if msg.author.id == owner_id {
            return CheckResult::Success;
        }
    }
    CheckResult::new_user("Command can only be used by guild owner.")
}

#[check]
#[name = "Config"]
#[check_in_help]
pub async fn config_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> CheckResult {
    let data = ctx.data.read().await;
    let pool = data.get::<Database>().unwrap();
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = i64::from(guild.id);

    let guild_data = sqlx::query!("SELECT * FROM guild_config WHERE guild_id = $1", guild_id)
        .fetch_optional(pool)
        .await;

    match guild_data {
        Err(why) => {
            error!("Error fetching guild_config from db: {}", why);
            return CheckResult::new_unknown();
        }
        Ok(data) => {
            if let Some(g) = data {
                if g.guild_id == guild_id {
                    return CheckResult::Success;
                }
            } else {
                if let Err(why) = msg.reply(&ctx.http, "I'm sorry but this feature is not enabled for this guild.\nPlease ask the owner of this guild to enable this feature by running the `config` command.").await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    CheckResult::new_unknown()
}
