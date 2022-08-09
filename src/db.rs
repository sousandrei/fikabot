use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use tracing::info;

use crate::Config;

pub mod channel;
pub mod user;

pub type DbConnection = sqlx::Pool<sqlx::MySql>;

pub async fn get_db(config: &Config) -> anyhow::Result<DbConnection> {
    info!("db connecting");

    let conn_options = MySqlConnectOptions::new()
        .username(&config.db_username)
        .password(&config.db_password)
        .host(&config.db_host)
        .port(config.db_port)
        .database(&config.db_database);

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(conn_options)
        .await?;

    info!("Connected to database");

    Ok(pool)
}
