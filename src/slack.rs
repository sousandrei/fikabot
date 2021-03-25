use serde::{Deserialize, Serialize};
use std::env;
use tracing::debug;

use crate::User;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackMessage {
    channel: String,
    text: String,
}

pub async fn send_message(user: &User, text: String) {
    let token = env::var("SLACK_TOKEN").unwrap();

    debug!("person {:#?} msg {:#?}", user.user_name, text);

    let User { user_id, .. } = user;

    let msg = SlackMessage {
        channel: format!("@{}", user_id),
        text,
    };

    reqwest::Client::new()
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(token)
        .json(&msg)
        .send()
        .await
        .unwrap();
}
