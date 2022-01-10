use std::env;

mod algos;
mod db;
mod http;
mod slack;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    check_env_vars();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    http::start().await?;

    Ok(())
}

fn check_env_vars() {
    let envs = [
        "SHEETS_ID",
        "ACCOUNT_EMAIL",
        "CREDENTIALS",
        "SLACK_TOKEN",
        "SLACK_SIGNING_SECRET",
        "WEBHOOK_TOKEN",
    ];

    for env in envs {
        let var = match env::var(env) {
            Ok(value) => value,
            Err(e) => panic!("{}: {}", e, env),
        };

        if var.is_empty() {
            panic!("{} is empty", env);
        }
    }
}
