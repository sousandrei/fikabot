use mongodb::bson::{doc, Bson};
use serde::{Deserialize, Serialize};
use std::env;

mod algos;
mod cron;
mod db;
mod http;
mod slack;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct User {
    user_id: String,
    user_name: String,
}

impl From<mongodb::bson::Document> for User {
    fn from(document: mongodb::bson::Document) -> Self {
        let user_id = document
            .get("user_id")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string();

        let user_name = document
            .get("user_name")
            .and_then(Bson::as_str)
            .unwrap()
            .to_string();

        User { user_id, user_name }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    cron::start();
    http::start().await?;

    Ok(())
}
