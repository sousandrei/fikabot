mod cron;
mod http;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    cron::spawn();
    http::start().await??;

    Ok(())
}
