use futures_util::stream::StreamExt;
use mongodb::{bson::doc, options::UpdateOptions, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Channel {
    pub channel_id: String,
    pub channel_name: String,
}

pub async fn add_channel(db: Database, channel: Channel) -> anyhow::Result<()> {
    let channels = db.collection::<Channel>("channels");

    let options = UpdateOptions::builder().upsert(true).build();

    channels
        .update_one(
            doc! { "channel_id": channel.channel_id.clone() },
            // TODO: impl this? gotta be a better way
            doc! {
                "channel_id": channel.channel_id.clone(),
                "channel_name": channel.channel_name.clone()
            },
            options,
        )
        .await?;

    Ok(())
}

pub async fn del_channel(db: Database, channel: Channel) -> anyhow::Result<()> {
    let channels = db.collection::<Channel>("channels");

    // TODO: filter error for channel not there
    channels
        .delete_one(doc! { "channel_id": channel.channel_id }, None)
        .await?;

    Ok(())
}

pub async fn list_channels(db: Database) -> anyhow::Result<Vec<Channel>> {
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
