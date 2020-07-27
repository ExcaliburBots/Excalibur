use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct GuildConfig {
    pub guild_id: i64,
    pub prefix: String,
}

impl GuildConfig {
    pub async fn get(
        guild_id: i64,
        default_prefix: &str,
        pool: &PgPool,
    ) -> Result<GuildConfig, Box<dyn std::error::Error>> {
        let guild =
            sqlx::query_as::<_, GuildConfig>("SELECT * FROM guild_config WHERE guild_id = $1")
                .bind(guild_id)
                .fetch_optional(pool)
                .await?;

        let mut guild_config: GuildConfig = GuildConfig {
            guild_id,
            prefix: String::from(default_prefix),
        };

        match guild {
            Some(g) => guild_config = g,
            None => {
                info!("Guild with id `{}` not in db. Adding to db...", guild_id);
                if let Err(why) = sqlx::query!(
                    "INSERT INTO guild_config (guild_id, prefix) VALUES($1, $2) ON CONFLICT DO NOTHING",
                    guild_id,
                    default_prefix
                ).execute(pool).await {
                    error!("Could not add the guild `{}` to db. Error: {}", guild_id, why);
                    return Err(Box::new(why));
                } else {
                    info!("Added guild to db.");
                }
            }
        }

        Ok(guild_config)
    }

    pub async fn set_prefix(
        &mut self,
        prefix: &str,
        pool: &PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.prefix = String::from(prefix);

        if let Err(why) = sqlx::query!(
            "UPDATE guild_config SET prefix = $2 WHERE guild_id = $1",
            self.guild_id,
            prefix
        )
        .execute(pool)
        .await
        {
            error!(
                "Could not set prefix for the guild `{}`. Error: {}",
                self.guild_id, why
            );
            return Err(Box::new(why));
        }

        Ok(())
    }
}
