use futures_util::stream::StreamExt;
use mongodb::{
    bson::{self, doc},
    options::UpdateOptions,
};
use serde::{Deserialize, Serialize};

use crate::db::get_db;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Channel {
    pub channel_id: String,
    pub channel_name: String,
}

impl Channel {
    pub async fn save(&self) -> anyhow::Result<()> {
        let db = get_db().await?;

        let channels = db.collection::<Channel>("channels");

        let options = UpdateOptions::builder().upsert(true).build();

        channels
            .update_one(
                doc! { "channel_id": self.channel_id.clone() },
                doc! { "$set": bson::to_document(self)? },
                options,
            )
            .await?;

        Ok(())
    }

    pub async fn delete(channel: &str) -> anyhow::Result<()> {
        let db = get_db().await?;

        let channels = db.collection::<Channel>("channels");

        // TODO: filter error for channel not there
        channels
            .delete_one(doc! { "channel_id": channel }, None)
            .await?;

        Ok(())
    }

    pub async fn list() -> anyhow::Result<Vec<Channel>> {
        let db = get_db().await?;

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
