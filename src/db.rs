use std::env;

use futures_util::stream::StreamExt;
use mongodb::{bson::doc, options::UpdateOptions, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Channel {
    pub channel_id: String,
    pub channel_name: String,
}

impl Channel {
    pub async fn save(&self) -> anyhow::Result<()> {
        // TODO: not crash hard if env is not here
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not present on environment");

        let client = Client::with_uri_str(&mongo_url).await?;
        let db = client.database("fika");

        let channels = db.collection::<Channel>("channels");

        let options = UpdateOptions::builder().upsert(true).build();

        channels
            .update_one(
                doc! { "channel_id": self.channel_id.clone() },
                doc! {
                    "$set": {
                        "channel_id": self.channel_id.clone(),
                        "channel_name": self.channel_name.clone()
                    }
                },
                options,
            )
            .await?;

        Ok(())
    }

    pub async fn del_channel(channel: &str) -> anyhow::Result<()> {
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not present on environment");

        let client = Client::with_uri_str(&mongo_url).await?;
        let db = client.database("fika");

        let channels = db.collection::<Channel>("channels");

        // TODO: filter error for channel not there
        channels
            .delete_one(doc! { "channel_id": channel }, None)
            .await?;

        Ok(())
    }

    pub async fn list_channels() -> anyhow::Result<Vec<Channel>> {
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not present on environment");

        let client = Client::with_uri_str(&mongo_url).await?;
        let db = client.database("fika");

        let channels = db.collection::<Channel>("channels");

        let channels = channels
            .find(None, None)
            .await?
            .filter_map(|channel| async move {
                match channel {
                    Ok(c) => Some(c),
                    // TODO: proper log errors
                    Err(_) => None,
                }
            })
            .collect::<Vec<Channel>>()
            .await;

        Ok(channels)
    }
}
