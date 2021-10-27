use rand::{prelude::SliceRandom, thread_rng};
use reusable_fmt::{fmt, fmt_reuse};
use tracing::info;

use crate::{
    db::user::User,
    slack::{self},
};

pub fn matchmake() -> anyhow::Result<()> {
    let mut users = User::list()?;

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
        message_pair(pairs[0])?;
        return Ok(());
    }

    // Send message to pairs
    for pair in pairs.iter().take(pairs.len() - 2) {
        message_pair(pair)?;
    }

    // If we have a trio, last pair is 1 person
    if pairs[pairs.len() - 1].len() < 2 {
        // Uses messages a trio
        message_trio(
            &pairs[pairs.len() - 1][0],
            &pairs[pairs.len() - 2][0],
            &pairs[pairs.len() - 2][1],
        )?;
    } else {
        // second to last pair
        message_pair(pairs[pairs.len() - 2])?;

        // Last pair
        message_pair(pairs[pairs.len() - 1])?;
    }

    Ok(())
}

// TODO: come up with a couple different message
fmt_reuse! {
    SONG = "song from is<@{}>!";
    SONG_LAST = "song from is<@{}>!, your song did not reach anyone,
     we'll try again next week or you can alwyas submit a new one :D";
}

fn message_pair(pair: &[User]) -> anyhow::Result<()> {
    if let [user1, user2] = pair {
        slack::send_message(&user1.user_id, fmt!(SONG, user1.user_id))?;
        slack::send_message(&user2.user_id, fmt!(SONG, user2.user_id))?;
    }

    Ok(())
}

fn message_trio(user1: &User, user2: &User, user3: &User) -> anyhow::Result<()> {
    slack::send_message(&user1.user_id, fmt!(SONG, user2.user_id))?;
    slack::send_message(&user2.user_id, fmt!(SONG, user1.user_id))?;
    slack::send_message(&user3.user_id, fmt!(SONG, user2.user_id))?;

    Ok(())
}
