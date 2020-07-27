use serenity::{
    framework::standard::{
        Args, CheckResult, CommandOptions, macros::check,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

#[check]
#[name = "GuildOwner"]
#[check_in_help]
pub async fn guild_owner_check(ctx: &Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    let guild = msg.guild(&ctx.cache).await;
    let owner_id = match &guild {
        Some(x) => x.owner_id,
        None => UserId(0)
    };
    if owner_id != 0 {
        if msg.author.id == owner_id {
            return CheckResult::Success;
        }
    }
    CheckResult::new_user("Command can only be used by guild owner.")
}