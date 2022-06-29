use std::env;

use serde::Deserialize;

mod algos;
mod db;
mod http;
mod slack;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    sheets_id: String,
    account_email: String,
    credentials: String,
    slack_token: String,
    slack_signing_secret: String,
    webhook_token: String,
    port: Option<u16>,
    env: Option<String>,
    rust_log: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };

    if config.rust_log.is_none() {
        env::set_var("RUST_LOG", "info");
    }

    if config.env.is_none() || config.env.as_ref().unwrap() != "prod" {
        tracing_subscriber::fmt::init();
    } else {
        tracing_subscriber::fmt().json().init();
    }

    http::start(&config).await?;

    Ok(())
}
