use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{db, slack, User};

pub async fn matchmake() {
    let mut users = db::list_users().await;

    // Shuffle people
    users.shuffle(&mut thread_rng());

    // Not enough people to pair
    if users.len() < 2 {
        return;
    }

    let pairs: Vec<&[User]> = users.chunks(2).collect();

    println!("len {}", pairs.len());

    if pairs.len() < 1 {
        return;
    }

    // Just one pair, handle naively
    if pairs.len() < 2 {
        slack::send_message(&pairs[0][0], "a").await;
        slack::send_message(&pairs[0][1], "a").await;

        return;
    }

    // Send message to pairs
    for i in 0..pairs.len() - 2 {
        let msg = "Here is your pair for the week normal";

        slack::send_message(&pairs[i][0], msg).await;
        slack::send_message(&pairs[i][1], msg).await;
    }

    if pairs[pairs.len() - 1].len() < 2 {
        let msg = "Here is your pair for this week. I had to include a second person since they were left of without a pair :c";
        // Last pair
        slack::send_message(&pairs[pairs.len() - 2][0], msg).await;
        slack::send_message(&pairs[pairs.len() - 2][1], msg).await;

        // Last person
        slack::send_message(&pairs[pairs.len() - 1][0], msg).await;
    } else {
        let msg = "Here is your pair for the week";

        // second to last pair
        slack::send_message(&pairs[pairs.len() - 2][0], msg).await;
        slack::send_message(&pairs[pairs.len() - 2][1], msg).await;

        // Last pair
        slack::send_message(&pairs[pairs.len() - 1][0], msg).await;
        slack::send_message(&pairs[pairs.len() - 1][1], msg).await;
    }
}
