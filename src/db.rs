use sea_orm::DatabaseConnection;
use tracing::info;

use crate::Config;

pub async fn get_db(config: &Config) -> anyhow::Result<DatabaseConnection> {
    info!("db connecting");

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database,
    );

    let connection = sea_orm::Database::connect(&url).await?;

    info!("Connected to database");

    Ok(connection)
}
