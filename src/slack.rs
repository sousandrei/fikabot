use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackMessage {
    channel: String,
    text: String,
}

pub fn send_message(user: &str, text: String) -> anyhow::Result<()> {
    let token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set");

    let msg = SlackMessage {
        channel: format!("@{}", user),
        text,
    };

    reqwest::blocking::Client::new()
        .post("https://slack.com/api/chat.postMessage")
        .bearer_auth(token)
        .json(&msg)
        .send()?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct ConversationsMembersResponse {
    members: Vec<String>,
    next_cursor: Option<String>,
}

pub fn get_channel_users(channel_id: &str) -> anyhow::Result<Vec<String>> {
    let token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set");

    let mut users = Vec::new();
    let mut cursor = None;

    loop {
        let mut params = vec![("channel", channel_id)];

        if cursor.is_some() {
            params.push(("cursor", cursor.as_deref().unwrap()))
        }

        let result = reqwest::blocking::Client::new()
            .post("https://slack.com/api/conversations.members")
            .bearer_auth(token.clone())
            .form(&params)
            .send()?;

        let ConversationsMembersResponse {
            mut members,
            next_cursor,
        }: ConversationsMembersResponse = result.json()?;

        users.append(&mut members);

        if next_cursor.is_none() {
            break;
        }

        cursor = next_cursor;
    }

    Ok(users)
}
