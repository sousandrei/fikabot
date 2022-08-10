use rand::{prelude::SliceRandom, thread_rng};
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::{error, info};

use entity::{channel, prelude::Channel};

use crate::slack::{self, get_channel_users};

pub async fn matchmake(config: &crate::Config, db: &DatabaseConnection) -> anyhow::Result<()> {
    let channels = Channel::find().all(db).await?;

    for channel in channels {
        if let Err(e) = matchmake_channel(&config.slack_token, &channel).await {
            error!("Error processing channel {:?}: {}", &channel, e);
        }
    }

    Ok(())
}

pub async fn matchmake_channel(token: &str, channel: &channel::Model) -> anyhow::Result<()> {
    info!("processing channel: {}", channel.name);

    let bot = slack::get_bot_id(token).await?;

    let mut users = get_channel_users(token, &channel.id).await?;

    users = users
        .into_iter()
        .filter(|u| u != &bot.bot_id && u != &bot.user_id)
        .collect();

    // Shuffle people
    users.shuffle(&mut thread_rng());

    // Not enough people to pair
    if users.len() < 2 {
        info!("not enough ppl");
        return Ok(());
    }

    info!("chunking pairs");
    let pairs: Vec<&[String]> = users.chunks(2).collect();

    if pairs.is_empty() {
        info!("empty pairs");
        return Ok(());
    }

    // Just one pair, handle naively
    if pairs.len() < 2 {
        info!("one pair");
        message_pair(token, &channel.id, pairs[0]).await?;
        return Ok(());
    }

    // Send message to pairs
    for pair in pairs.iter().take(pairs.len() - 2) {
        message_pair(token, &channel.id, pair).await?;
    }

    // If we have a trio, last pair is 1 person
    if pairs[pairs.len() - 1].len() < 2 {
        info!("one trio");

        // Uses messages a trio
        message_trio(
            token,
            &channel.name,
            &pairs[pairs.len() - 1][0],
            &pairs[pairs.len() - 2][0],
            &pairs[pairs.len() - 2][1],
        )
        .await?;
    } else {
        info!("two last pairs");

        // second to last pair
        message_pair(token, &channel.id, pairs[pairs.len() - 2]).await?;

        // Last pair
        message_pair(token, &channel.id, pairs[pairs.len() - 1]).await?;
    }

    Ok(())
}

// TODO: come up with a couple different message

pub async fn message_pair(token: &str, channel: &str, pair: &[String]) -> anyhow::Result<()> {
    let msg = |channel: &str, user: &str| {
        format!("This week your fika pair for channel `{channel}` is <@{user}>!")
    };

    if let [user1, user2] = pair {
        slack::send_message(token, user1, msg(channel, user2)).await?;
        slack::send_message(token, user2, msg(channel, user1)).await?;
    }

    Ok(())
}

pub async fn message_trio(
    token: &str,
    channel: &str,
    user1: &str,
    user2: &str,
    user3: &str,
) -> anyhow::Result<()> {
    let msg = |channel: &str, user1: &str, user2: &str| {
        format!(
            "This week your fika \"pair\"(s) for channel `{channel}` are <@{user1}> and <@{user2}>!
This time you got an extra buddy! ;)"
        )
    };

    slack::send_message(token, user1, msg(channel, user2, user3)).await?;
    slack::send_message(token, user2, msg(channel, user1, user3)).await?;
    slack::send_message(token, user3, msg(channel, user1, user2)).await?;

    Ok(())
}
