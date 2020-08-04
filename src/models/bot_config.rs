use colored::Colorize;
use serde::Deserialize;
use serenity::model::id::UserId;
use std::collections::HashSet;
use std::process;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub owner: Option<Vec<OwnerConfig>>,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    default_prefix: Option<String>,
    id: Option<u64>,
    invite_url: Option<String>,
    token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OwnerConfig {
    id: u64,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    database_url: Option<String>,
    name: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    address: Option<String>,
}

impl Config {
    pub fn bot_token(&self) -> String {
        if let Some(token) = &self.bot.token {
            String::from(token)
        } else {
            println!("{}", "Bot token is not declared in the config file.".red());
            process::exit(0);
        }
    }

    pub fn bot_default_prefix(&self) -> String {
        if let Some(prefix) = &self.bot.default_prefix {
            String::from(prefix)
        } else {
            println!("{}", "Bot prefix is not declared in the config file.".red());
            process::exit(0);
        }
    }

    pub fn bot_id(&self) -> UserId {
        if let Some(id) = self.bot.id {
            UserId(id)
        } else {
            println!("{}", "Bot id is not declared in the config file.".red());
            process::exit(0);
        }
    }

    pub fn invite_url(&self) -> String {
        if let Some(url) = &self.bot.invite_url {
            String::from(url)
        } else {
            println!(
                "{}",
                "Bot invite url is not declared in the config file.".red()
            );
            process::exit(0);
        }
    }

    pub fn get_owners(&self) -> HashSet<UserId> {
        let mut owners = HashSet::new();

        if let Some(owner_config) = &self.owner {
            for owner in owner_config {
                owners.insert(UserId(owner.id));
            }
        }

        owners
    }
}

impl DatabaseConfig {
    pub fn get_database_url(&self) -> String {
        if let Some(url) = &self.database_url {
            return String::from(url);
        }

        let default_port: u16 = 5432;
        let mut url: (&str, &str, &str, &u16, &str) = ("", "", "localhost", &default_port, "");

        if let Some(username) = &self.username {
            url.0 = username;
        }
        if let Some(pwd) = &self.password {
            url.1 = pwd;
        }
        if let Some(address) = &self.address {
            url.2 = address;
        }
        if let Some(port) = &self.port {
            url.3 = port;
        }
        if let Some(name) = &self.name {
            url.4 = name;
            return String::from(format!(
                "postgresql://{}:{}@{}:{}/{}",
                url.0, url.1, url.2, url.3, url.4
            ));
        }

        println!("{}", "Database settings not set.".red());
        process::exit(0);
    }
}
