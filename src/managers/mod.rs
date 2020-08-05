use std::collections::HashMap;
use std::sync::Arc;

use serenity::client::bridge::gateway::ShardManager;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::TypeMapKey;
use sqlx::PgPool;
use tokio::sync::Mutex;

use crate::models::bot_config::Config;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct Database;

impl TypeMapKey for Database {
    type Value = PgPool;
}

pub struct DefaultPrefix;

impl TypeMapKey for DefaultPrefix {
    type Value = String;
}

pub struct GuildPrefix;

impl TypeMapKey for GuildPrefix {
    type Value = HashMap<u64, String>;
}

pub struct BotConfig;

impl TypeMapKey for BotConfig {
    type Value = Config;
}
