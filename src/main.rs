use std::env;

mod algos;
mod cron;
mod db;
mod http;
mod slack;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    cron::start();
    http::start().await?;

    Ok(())
}

// CEDPJBW8L
