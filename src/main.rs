use serde::Deserialize;
use std::env;

mod algos;
mod db;
mod http;
mod slack;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    slack_token: String,
    slack_signing_secret: String,

    port: Option<u16>,
    webhook_token: String,

    db_username: String,
    db_password: String,
    db_host: String,
    db_port: u16,
    db_database: String,

    env: Option<String>,
    rust_log: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{error:#?}"),
    };

    if config.rust_log.is_none() {
        env::set_var("RUST_LOG", "info");
    }

    if config.env.is_none() || config.env.as_ref().unwrap() != "prod" {
        tracing_subscriber::fmt::init();
    } else {
        tracing_subscriber::fmt().json().init();
    }

    let db = db::get_db(&config).await?;

    http::start(&config, &db).await?;

    Ok(())
}
