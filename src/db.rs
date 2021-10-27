use mongodb::sync::{Client, Database};
use std::env;

pub mod channel;
pub mod user;

fn get_db() -> anyhow::Result<Database> {
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not present on environment");

    let client = Client::with_uri_str(&mongo_url)?;
    let db = client.database("fika");

    Ok(db)
}
