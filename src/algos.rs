use mongodb::Client;
use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    db,
    db::{Channel, User},
    slack,
};

pub async fn matchmake() -> anyhow::Result<()> {
    // TODO: abstract DB / move from mongo?
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db = client.database("fika");

    let channels = db::list_channels(db).await?;

    for channel in channels {
        let mut users = get_channels_users(channel).await?;

        // Shuffle people
        users.shuffle(&mut thread_rng());

        // Not enough people to pair
        if users.len() < 2 {
            return Ok(());
        }

        let pairs: Vec<&[User]> = users.chunks(2).collect();

        if pairs.is_empty() {
            return Ok(());
        }

        // Just one pair, handle naively
        if pairs.len() < 2 {
            message_pair(pairs[0]).await;
            return Ok(());
        }

        // Send message to pairs
        for pair in pairs.iter().take(pairs.len() - 2) {
            message_pair(pair).await;
        }

        if pairs[pairs.len() - 1].len() < 2 {
            message_trio(
                &pairs[pairs.len() - 1][0],
                &pairs[pairs.len() - 2][0],
                &pairs[pairs.len() - 2][1],
            )
            .await;
        } else {
            // second to last pair
            message_pair(pairs[pairs.len() - 2]).await;

            // Last pair
            message_pair(pairs[pairs.len() - 1]).await;
        }
    }

    Ok(())
}

async fn get_channels_users(_channel: Channel) -> anyhow::Result<Vec<User>> {
    // TODO: get users from slack
    let users = Vec::new();

    Ok(users)
}

// TODO: come up with a couple different message
async fn message_pair(pair: &[User]) {
    if let [user1, user2] = pair {
        slack::send_message(
            user1,
            format!("This week your fika pair is<@{}>!", user1.user_id),
        )
        .await;

        slack::send_message(
            user2,
            format!("This week your fika pair is<@{}>!", user2.user_id),
        )
        .await;
    }
}

async fn message_trio(user1: &User, user2: &User, user3: &User) {
    slack::send_message(
        user1,
        format!(
            "Here is your pair for this week. <@{}> and <@{}>!\nThis time you got an extra buddy! ;)",
            user2.user_id,
            user3.user_id
        ),
    )
    .await;

    slack::send_message(
        user2,
        format!(
            "Here is your pair for this week. <@{}> and <@{}>!\nThis time you got an extra buddy! ;)",
            user1.user_id,
            user3.user_id
        ),
    )
    .await;

    slack::send_message(
        user3,
        format!(
            "Here is your pair for this week. <@{}> and <@{}>!\nThis time you got an extra buddy! ;)",
            user1.user_id,
            user2.user_id
        ),
    )
    .await;
}
