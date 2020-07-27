use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;
use serenity::client::bridge::gateway::ShardManager;
use serenity::client::bridge::voice::ClientVoiceManager;
use sqlx::PgPool;

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