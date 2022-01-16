use rand::{prelude::SliceRandom, thread_rng};
use reusable_fmt::{fmt, fmt_reuse};
use tracing::info;

use crate::{
    db::user::User,
    slack::{self},
};

pub async fn matchmake(config: &crate::Config) -> anyhow::Result<()> {
    let mut users = User::list(config).await?;

    let bot = slack::get_bot_id(&config.slack_token).await?;

    users = users
        .into_iter()
        .filter(|u| u.user_id != bot.bot_id && u.user_id != bot.user_id)
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

// TODO: come up with a couple different message
fmt_reuse! {
    SONG = "song from is<@{}>!";
    SONG_LAST = "song from is<@{}>!, your song did not reach anyone,
     we'll try again next week or you can alwyas submit a new one :D";
}

async fn message_pair(token: &str, pair: &[User]) -> anyhow::Result<()> {
    if let [user1, user2] = pair {
        slack::send_message(token, &user1.user_id, fmt!(SONG, user1.user_id)).await?;
        slack::send_message(token, &user2.user_id, fmt!(SONG, user2.user_id)).await?;
    }

    Ok(())
}

async fn message_trio(token: &str, user1: &User, user2: &User, user3: &User) -> anyhow::Result<()> {
    slack::send_message(token, &user1.user_id, fmt!(SONG, user2.user_id)).await?;
    slack::send_message(token, &user2.user_id, fmt!(SONG, user1.user_id)).await?;
    slack::send_message(token, &user3.user_id, fmt!(SONG, user2.user_id)).await?;

    Ok(())
}
