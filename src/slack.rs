use serde::{Deserialize, Serialize};
use std::env;
use tracing::debug;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackMessage {
    channel: String,
    text: String,
}

pub async fn send_message(user: &str, text: String) -> anyhow::Result<()> {
    let token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set");

    debug!("person {:#?} msg {:#?}", user, text);

    let msg = SlackMessage {
        channel: format!("@{}", user),
        text,
    };

    reqwest::Client::new()
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(token)
        .json(&msg)
        .send()
        .await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct ConversationsMembersResponse {
    members: Vec<String>,
    next_cursor: Option<String>,
}

pub async fn get_channel_users(channel_id: &str) -> anyhow::Result<Vec<String>> {
    let token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set");

    let mut users = Vec::new();
    let mut cursor = None;

    loop {
        let mut params = vec![("channel", channel_id)];

        if cursor.is_some() {
            params.push(("cursor", cursor.as_deref().unwrap()))
        }

        let result = reqwest::Client::new()
            .post("https://slack.com/api/conversations.members")
            .bearer_auth(token.clone())
            .form(&params)
            .send()
            .await
            .unwrap();

        let ConversationsMembersResponse {
            mut members,
            next_cursor,
        }: ConversationsMembersResponse = result.json().await?;

        users.append(&mut members);

        if next_cursor.is_none() {
            break;
        }

        cursor = next_cursor;
    }

    Ok(users)
}
