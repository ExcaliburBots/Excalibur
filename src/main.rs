use std::{collections::HashSet, fs, sync::Arc};

extern crate serenity;
use serenity::prelude::*;
use serenity::{
    async_trait,
    framework::standard::{
        help_commands,
        macros::{group, help, hook},
        Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
    },
    model::{channel::Message, event::ResumedEvent, gateway::Ready, id::UserId},
};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod commands;
use commands::{checks::*, config::*, general::*, moderation::*, owner::*, voice::*};

mod managers;
use crate::models::guild_config::GuildConfig;
use managers::*;
use std::time::Duration;
use tokio::time::delay_for;

mod models;
mod utils;

// Event Handler

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.
            info!(
                "{} is connected on shard {}/{}!",
                ready.user.name,
                shard[0] + 1,
                shard[1],
            );
        }
        // info!("{} is connected!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

// Groups

#[group]
#[commands(about, say, ping)]
struct General;

#[group]
#[only_in(guilds)]
#[commands(slow_mode, ban)]
struct Moderation;

#[group]
#[only_in(guilds)]
#[checks(Config)]
#[commands(deafen, join, leave, mute, play, undeafen, unmute)]
struct Voice;

#[group]
#[only_in(guilds)]
#[commands(config)]
struct Config;

#[group]
#[owners_only]
#[commands(quit)]
struct Owner;

// Help Command

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

// Hooks

#[hook]
async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let data = ctx.data.read().await;
    let pool = data.get::<Database>().unwrap();
    let default_prefix = data.get::<DefaultPrefix>().unwrap();
    let guild_id = msg.guild(&ctx.cache).await.unwrap().id;

    let prefix = GuildConfig::get_prefix(guild_id, default_prefix.to_string(), pool)
        .await
        .unwrap();

    Some(prefix)
}

// Main

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kankyo::load().expect("Failed to load .env file");
    pretty_env_logger::init();

    let config_file = fs::read_to_string("config.toml").unwrap();
    let bot_config: models::bot_config::Config = toml::from_str(&config_file).unwrap();

    let owners = bot_config.get_owners();
    let bot_id = bot_config.bot_id();
    let token = bot_config.bot_token();

    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // let http = Http::new_with_token(&token);

    // let (owners, bot_id) = match http.get_current_application_info().await {
    //     Ok(info) => {
    //         let mut owners = HashSet::new();
    //
    //         for team in info.team {
    //             for team_member in team.members {
    //                 owners.insert(team_member.user.id);
    //             }
    //         }
    //
    //         (owners, info.id)
    //     }
    //     Err(why) => panic!("Could not access application info: {:?}", why),
    // };

    let pool = utils::obtain_pool(&*bot_config.database.get_database_url()).await?;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .dynamic_prefix(dynamic_prefix)
                .delimiters(vec![", ", ","])
                .owners(owners)
        })
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MODERATION_GROUP)
        .group(&VOICE_GROUP)
        .group(&CONFIG_GROUP)
        .group(&OWNER_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Database>(pool.clone());
        data.insert::<DefaultPrefix>(bot_config.bot_default_prefix());
        data.insert::<BotConfig>(bot_config);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }

    let manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop {
            delay_for(Duration::from_secs(120)).await;

            let lock = manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                info!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id, runner.stage, runner.latency,
                );
            }
        }
    });

    if let Err(why) = client.start_shards(2).await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
