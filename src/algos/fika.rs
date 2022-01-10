use reusable_fmt::{fmt, fmt_reuse};

use rand::{prelude::SliceRandom, thread_rng};
use tracing::{error, info};

use crate::{
    db::channel::Channel,
    slack::{self, get_channel_users},
};

pub async fn matchmake() -> anyhow::Result<()> {
    let channels = Channel::list().await?;

    for channel in channels {
        if let Err(e) = matchmake_channel(&channel) {
            error!("Error processing channel {:?}: {}", &channel, e);
        }
    }

    Ok(())
}

pub fn matchmake_channel(channel: &Channel) -> anyhow::Result<()> {
    info!("processing channel: {}", channel.channel_name);

    let bot = slack::get_bot_id().await?;

    let mut users = get_channel_users(&channel.channel_id)?;

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
        message_pair(channel, pairs[0])?;
        return Ok(());
    }

    // Send message to pairs
    for pair in pairs.iter().take(pairs.len() - 2) {
        message_pair(channel, pair)?;
    }

    // If we have a trio, last pair is 1 person
    if pairs[pairs.len() - 1].len() < 2 {
        info!("one trio");

        // Uses messages a trio
        message_trio(
            channel,
            &pairs[pairs.len() - 1][0],
            &pairs[pairs.len() - 2][0],
            &pairs[pairs.len() - 2][1],
        )?;
    } else {
        info!("two last pairs");

        // second to last pair
        message_pair(channel, pairs[pairs.len() - 2])?;

        // Last pair
        message_pair(channel, pairs[pairs.len() - 1])?;
    }

    Ok(())
}

// TODO: come up with a couple different message
fmt_reuse! {
    FIKA_PAIR = "This week your fika pair for channel `{}` is <@{}>!";
    FIKA_TRIO = "This week your fika \"pair\"(s) for channel `{}` are <@{}> and <@{}>!\nThis time you got an extra buddy! ;)";
}

pub fn message_pair(channel: &Channel, pair: &[String]) -> anyhow::Result<()> {
    if let [user1, user2] = pair {
        slack::send_message(user1, fmt!(FIKA_PAIR, channel.channel_name, user2))?;
        slack::send_message(user2, fmt!(FIKA_PAIR, channel.channel_name, user1))?;
    }

    Ok(())
}

pub fn message_trio(
    channel: &Channel,
    user1: &str,
    user2: &str,
    user3: &str,
) -> anyhow::Result<()> {
    slack::send_message(user1, fmt!(FIKA_TRIO, channel.channel_name, user2, user3))?;
    slack::send_message(user2, fmt!(FIKA_TRIO, channel.channel_name, user1, user3))?;
    slack::send_message(user3, fmt!(FIKA_TRIO, channel.channel_name, user1, user2))?;

    Ok(())
}
