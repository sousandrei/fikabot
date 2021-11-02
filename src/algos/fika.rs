use reusable_fmt::{fmt, fmt_reuse};

use rand::{prelude::SliceRandom, thread_rng};
use tracing::info;

use crate::{
    db::channel::Channel,
    slack::{self, get_channel_users},
};

pub fn matchmake() -> anyhow::Result<()> {
    let channels = Channel::list()?;

    for channel in channels {
        info!("processing channel: {}", channel.channel_name);

        let mut users = get_channel_users(&channel.channel_id)?;

        // Shuffle people
        users.shuffle(&mut thread_rng());

        // Not enough people to pair
        if users.len() < 2 {
            info!("not enough ppl");
            continue;
        }

        info!("chunking pairs");
        let pairs: Vec<&[String]> = users.chunks(2).collect();

        if pairs.is_empty() {
            info!("empty pairs");
            continue;
        }

        // Just one pair, handle naively
        if pairs.len() < 2 {
            info!("one pair");
            message_pair(&channel, pairs[0])?;
            continue;
        }

        // Send message to pairs
        for pair in pairs.iter().take(pairs.len() - 2) {
            message_pair(&channel, pair)?;
        }

        // If we have a trio, last pair is 1 person
        if pairs[pairs.len() - 1].len() < 2 {
            info!("one trio");

            // Uses messages a trio
            message_trio(
                &channel,
                &pairs[pairs.len() - 1][0],
                &pairs[pairs.len() - 2][0],
                &pairs[pairs.len() - 2][1],
            )?;
        } else {
            info!("two last pairs");

            // second to last pair
            message_pair(&channel, pairs[pairs.len() - 2])?;

            // Last pair
            message_pair(&channel, pairs[pairs.len() - 1])?;
        }
    }

    Ok(())
}

// TODO: come up with a couple different message
fmt_reuse! {
    FIKA_PAIR = "This week your fika pair for channel `{}` is<@{}>!";
    FIKA_TRIO = "This week your fika \"pair\"(s) for channel `{}` are <@{}> and <@{}>!\nThis time you got an extra buddy! ;)";
}

pub fn message_pair(channel: &Channel, pair: &[String]) -> anyhow::Result<()> {
    if let [user1, user2] = pair {
        slack::send_message(user1, fmt!(FIKA_PAIR, user1, channel.channel_name))?;
        slack::send_message(user2, fmt!(FIKA_PAIR, user2, channel.channel_name))?;
    }

    Ok(())
}

pub fn message_trio(
    channel: &Channel,
    user1: &str,
    user2: &str,
    user3: &str,
) -> anyhow::Result<()> {
    slack::send_message(user1, fmt!(FIKA_TRIO, user2, user3, channel.channel_name))?;
    slack::send_message(user2, fmt!(FIKA_TRIO, user1, user3, channel.channel_name))?;
    slack::send_message(user3, fmt!(FIKA_TRIO, user1, user2, channel.channel_name))?;

    Ok(())
}
