use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

use crate::Config;

pub async fn get_db(config: &Config) -> anyhow::Result<DatabaseConnection> {
    info!("db connection initializing");

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username, config.db_password, config.db_host, config.db_port, config.db_database,
    );

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(3).sqlx_logging(false);
    let connection = Database::connect(opt).await?;

    info!("connected to database");

    Ok(connection)
}
