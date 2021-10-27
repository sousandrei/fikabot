use mongodb::{
    bson::{self, doc},
    options::UpdateOptions,
};
use serde::{Deserialize, Serialize};

use crate::db::get_db;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub song: String,
}

impl User {
    pub fn save(&self) -> anyhow::Result<()> {
        let db = get_db()?;

        let users = db.collection::<User>("users");

        let options = UpdateOptions::builder().upsert(true).build();

        users.update_one(
            doc! { "user_id": self.user_id.clone() },
            doc! { "$set": bson::to_document(self)? },
            options,
        )?;

        Ok(())
    }

    pub fn _delete(user: &str) -> anyhow::Result<()> {
        let db = get_db()?;

        let users = db.collection::<User>("users");

        // TODO: filter error for channel not there
        users.delete_one(doc! { "user_id": user }, None)?;

        Ok(())
    }

    pub fn list() -> anyhow::Result<Vec<User>> {
        let db = get_db()?;

        let users = db.collection::<User>("users");

        let result = users
            .find(None, None)?
            .filter_map(|channel| {
                match channel {
                    Ok(c) => Some(c),
                    // TODO: proper log errors
                    Err(_) => None,
                }
            })
            .collect::<Vec<User>>();

        Ok(result)
    }
}
