use std::env;

use serde::{Deserialize, Serialize};

mod algos;
mod cron;
mod db;
mod http;
mod slack;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct User {
    user_id: String,
    user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    cron::start();
    http::start().await;

    Ok(())
}
