use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::env;
use tide::StatusCode;
use tracing::info;

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

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

pub fn verify_slack(expt_sign: &str, ts: &str, body: &str) -> Result<(), StatusCode> {
    let signing_secret =
        env::var("SLACK_SIGNING_SECRET").expect("SLACK_SIGNING_SECRET not present");

    // To verify the message:
    let mut mac = match HmacSha256::new_from_slice(signing_secret.as_bytes()) {
        Ok(mac) => mac,
        Err(e) => {
            info!("canot start hmacsha256: {}", e);
            return Err(StatusCode::InternalServerError);
        }
    };

    let signature_base = format!("v0:{}:{}", ts, body);

    mac.update(signature_base.as_bytes());

    let sig = format!("v0={}", hex::encode(mac.finalize().into_bytes()));

    if sig != expt_sign {
        return Err(StatusCode::Unauthorized);
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Bot {
    pub bot_id: String,
    pub user_id: String,
}

pub async fn get_bot_id() -> anyhow::Result<Bot> {
    let token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN not set");

    let result = reqwest::Client::new()
        .post("https://slack.com/api/auth.test")
        .bearer_auth(token.clone())
        .send()
        .await?;

    let bot: Bot = result.json().await?;
    Ok(bot)
}
