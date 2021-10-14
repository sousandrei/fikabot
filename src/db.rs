use mongodb::{bson::doc, options::UpdateOptions, sync::Database};

use crate::User;

pub async fn add_user(db: Database, user: User) -> anyhow::Result<()> {
    let users = db.collection("users");

    let options = UpdateOptions::builder().upsert(true).build();

    // TODO: different message for user that is being updated!
    users.update_one(
        doc! { "user_id": user.user_id.clone() },
        // TODO: impl user to doc
        doc! {
            "user_id": user.user_id,
            "user_name": user.user_name
        },
        options,
    )?;

    Ok(())
}

pub async fn del_user(db: Database, user: User) -> anyhow::Result<()> {
    let users = db.collection("users");

    // TODO: return error if user is not here (different message)
    users.delete_one(doc! { "user_id": user.user_id }, None)?;

    Ok(())
}

pub async fn list_users(db: Database) -> anyhow::Result<Vec<User>> {
    let users = db.collection("users");

    let users: Vec<User> = users
        .find(None, None)?
        .filter_map(|document| {
            if document.is_err() {
                return None;
            }

            let document = document.unwrap();
            Some(document.into())
        })
        .collect();

    Ok(users)
}
