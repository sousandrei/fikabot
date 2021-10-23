use mongodb::{bson::doc, options::UpdateOptions};
use serde::{Deserialize, Serialize};

use crate::db::get_db;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub song: String,
}

impl User {
    pub async fn save(&self) -> anyhow::Result<()> {
        let db = get_db().await?;

        let users = db.collection::<User>("users");

        let options = UpdateOptions::builder().upsert(true).build();

        users
            .update_one(
                doc! { "user_id": self.user_id.clone() },
                doc! {
                    "$set": {
                        "user_id": self.user_id.clone(),
                        "user_name": self.user_name.clone()
                    }
                },
                options,
            )
            .await?;

        Ok(())
    }

    pub async fn _delete(user: &str) -> anyhow::Result<()> {
        let db = get_db().await?;

        let users = db.collection::<User>("users");

        // TODO: filter error for channel not there
        users.delete_one(doc! { "user_id": user }, None).await?;

        Ok(())
    }
}
