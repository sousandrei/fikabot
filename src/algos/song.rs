use rand::{prelude::SliceRandom, thread_rng};
use tracing::info;

use crate::{
    db::user::User,
    slack::{self},
};

pub async fn matchmake(config: &crate::Config, db: &crate::db::DbConnection) -> anyhow::Result<()> {
    let mut users = User::find_all(db).await?;

    let bot = slack::get_bot_id(&config.slack_token).await?;

    users = users
        .into_iter()
        .filter(|u| u.id != bot.bot_id && u.id != bot.user_id)
        .collect();

    // Shuffle people
    users.shuffle(&mut thread_rng());

    // Not enough people to pair
    if users.len() < 2 {
        info!("not enough ppl");
        return Ok(());
    }

    info!("chunking pairs");
    let pairs: Vec<&[User]> = users.chunks(2).collect();

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

async fn message_pair(token: &str, pair: &[User]) -> anyhow::Result<()> {
    let msg = |user: &str| format!("song from is<@{user}>!");

    if let [user1, user2] = pair {
        slack::send_message(token, &user1.id, msg(&user1.id)).await?;
        slack::send_message(token, &user2.id, msg(&user2.id)).await?;
    }

    Ok(())
}

async fn message_trio(token: &str, user1: &User, user2: &User, user3: &User) -> anyhow::Result<()> {
    let msg = |user: &str| {
        format!(
            "song from is<@{user}>!, your song did not reach anyone,
    we'll try again next week or you can alwyas submit a new one :D"
        )
    };

    slack::send_message(token, &user1.id, msg(&user2.id)).await?;
    slack::send_message(token, &user2.id, msg(&user1.id)).await?;
    slack::send_message(token, &user3.id, msg(&user2.id)).await?;

    Ok(())
}
