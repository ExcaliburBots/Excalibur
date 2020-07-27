use serenity::client::bridge::gateway::ShardManager;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::prelude::TypeMapKey;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub(crate) struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub(crate) struct Database;

impl TypeMapKey for Database {
    type Value = PgPool;
}

pub(crate) struct DefaultPrefix;

impl TypeMapKey for DefaultPrefix {
    type Value = String;
}
