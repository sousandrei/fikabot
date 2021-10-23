use reusable_fmt::{fmt, fmt_reuse};

use rand::{prelude::SliceRandom, thread_rng};
use tracing::info;

use crate::{
    db::channel::Channel,
    slack::{self, get_channel_users},
};

pub async fn matchmake() -> anyhow::Result<()> {
    let channels = Channel::list().await?;

    for channel in channels {
        let mut users = get_channel_users(&channel.channel_id).await?;

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
            message_pair(pairs[0]).await?;
            continue;
        }

        // Send message to pairs
        for pair in pairs.iter().take(pairs.len() - 2) {
            message_pair(pair).await?;
        }

        // If we have a trio, last pair is 1 person
        if pairs[pairs.len() - 1].len() < 2 {
            // Uses messages a trio
            message_trio(
                &pairs[pairs.len() - 1][0],
                &pairs[pairs.len() - 2][0],
                &pairs[pairs.len() - 2][1],
            )
            .await?;
        } else {
            // second to last pair
            message_pair(pairs[pairs.len() - 2]).await?;

            // Last pair
            message_pair(pairs[pairs.len() - 1]).await?;
        }
    }

    Ok(())
}

// TODO: come up with a couple different message
fmt_reuse! {
    FIKA_PAIR = "This week your fika pair is<@{}>!";
    FIKA_TRIO = "Here is your \"pair\" for this week. <@{}> and <@{}>!\nThis time you got an extra buddy! ;)";
}

pub async fn message_pair(pair: &[String]) -> anyhow::Result<()> {
    if let [user1, user2] = pair {
        slack::send_message(user1, fmt!(FIKA_PAIR, user1)).await?;
        slack::send_message(user2, fmt!(FIKA_PAIR, user2)).await?;
    }

    Ok(())
}

pub async fn message_trio(user1: &str, user2: &str, user3: &str) -> anyhow::Result<()> {
    slack::send_message(user1, fmt!(FIKA_TRIO, user2, user3)).await?;
    slack::send_message(user2, fmt!(FIKA_TRIO, user1, user3)).await?;
    slack::send_message(user3, fmt!(FIKA_TRIO, user1, user2)).await?;

    Ok(())
}
