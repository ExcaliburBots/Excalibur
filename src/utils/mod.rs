use std::error::Error;

use serenity::model::id::GuildId;
use serenity::prelude::Context;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::managers::{Database, DefaultPrefix, GuildPrefix};
use crate::models::guild_config::GuildConfig;

pub async fn obtain_pool(url: &str) -> Result<PgPool, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(10) // maximum number of connections in the pool
        .connect(url)
        .await?;

    Ok(pool)
}

pub async fn save_prefix(prefix: String, guild_id: GuildId, ctx: &Context) {
    let data = ctx.data.read().await;
    let default_prefix = data.get::<DefaultPrefix>().unwrap();
    let pool = data.get::<Database>().unwrap();

    // saving prefix in db
    let mut guild_config = GuildConfig::get(guild_id.0 as i64, default_prefix, pool)
        .await
        .unwrap();
    guild_config
        .set_prefix(prefix.as_str(), pool)
        .await
        .unwrap();

    // saving prefix in shared data
    let ctx = ctx.clone();
    let save_prefix = String::from(&prefix);

    tokio::spawn(async move {
        let mut data = ctx.data.write().await;
        let guild = data.get_mut::<GuildPrefix>().unwrap();
        guild.insert(guild_id.0, save_prefix);
    });
}

pub async fn get_default_prefix(ctx: &Context) -> String {
    let data = ctx.data.read().await;
    let default_prefix = data.get::<DefaultPrefix>().unwrap();

    String::from(default_prefix)
}
