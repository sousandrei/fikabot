use rand::{prelude::SliceRandom, thread_rng};
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::info;

use entity::prelude::User;

use crate::slack;

pub async fn matchmake(config: &crate::Config, db: &DatabaseConnection) -> anyhow::Result<()> {
    let users = User::find().all(db).await?;
    let bot = slack::get_bot_id(&config.slack_token).await?;

    let mut users_ids: Vec<String> = users
        .into_iter()
        .filter_map(|u| {
            if u.id != bot.bot_id && u.id != bot.user_id {
                Some(u.id)
            } else {
                None
            }
        })
        .collect();

    // Shuffle people
    users_ids.shuffle(&mut thread_rng());

    // Not enough people to pair
    if users_ids.len() < 2 {
        info!("not enough ppl");
        return Ok(());
    }

    info!("chunking pairs");
    let pairs: Vec<&[String]> = users_ids.chunks(2).collect();

    if pairs.is_empty() {
        info!("empty pairs");
        return Ok(());
    }

    // Just one pair, handle naively
    if pairs.len() < 2 {
        info!("one pair");
        message_pair(&config.slack_token, pairs[0]).await?;
        return Ok(());
    }

    // Send message to pairs
    for pair in pairs.iter().take(pairs.len() - 2) {
        message_pair(&config.slack_token, pair).await?;
    }

    // If we have a trio, last pair is 1 person
    if pairs[pairs.len() - 1].len() < 2 {
        // Uses messages a trio
        message_trio(
            &config.slack_token,
            &pairs[pairs.len() - 1][0],
            &pairs[pairs.len() - 2][0],
            &pairs[pairs.len() - 2][1],
        )
        .await?;
    } else {
        // second to last pair
        message_pair(&config.slack_token, pairs[pairs.len() - 2]).await?;

        // Last pair
        message_pair(&config.slack_token, pairs[pairs.len() - 1]).await?;
    }

    Ok(())
}

// TODO: come up with a couple different messages

async fn message_pair(token: &str, pair: &[String]) -> anyhow::Result<()> {
    let msg = |user: &str| format!("song from is<@{user}>!");

    if let [user1, user2] = pair {
        slack::send_message(token, user1, msg(user1)).await?;
        slack::send_message(token, user2, msg(user2)).await?;
    }

    Ok(())
}

async fn message_trio(token: &str, user1: &str, user2: &str, user3: &str) -> anyhow::Result<()> {
    let msg = |user: &str| {
        format!(
            "song from is<@{user}>!, your song did not reach anyone,
    we'll try again next week or you can alwyas submit a new one :D"
        )
    };

    slack::send_message(token, user1, msg(user2)).await?;
    slack::send_message(token, user2, msg(user1)).await?;
    slack::send_message(token, user3, msg(user2)).await?;

    Ok(())
}
