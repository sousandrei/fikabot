use axum::http::StatusCode;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tracing::info;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackMessage {
    channel: String,
    text: String,
}

pub async fn send_message(token: &str, user: &str, text: String) -> anyhow::Result<()> {
    let msg = SlackMessage {
        channel: format!("@{user}"),
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

pub async fn get_channel_users(token: &str, channel_id: &str) -> anyhow::Result<Vec<String>> {
    let mut users = Vec::new();
    let mut cursor = None;

    loop {
        let mut params = vec![("channel", channel_id)];

        if cursor.is_some() {
            params.push(("cursor", cursor.as_deref().unwrap()))
        }

        let result = reqwest::Client::new()
            .post("https://slack.com/api/conversations.members")
            .bearer_auth(token)
            .form(&params)
            .send()
            .await?;

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

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

pub fn verify_slack(
    signing_secret: &str,
    expt_sign: &str,
    ts: &str,
    body: &str,
) -> Result<(), StatusCode> {
    // To verify the message:
    let mut mac = match HmacSha256::new_from_slice(signing_secret.as_bytes()) {
        Ok(mac) => mac,
        Err(e) => {
            info!("canot start hmacsha256: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let signature_base = format!("v0:{ts}:{body}");

    mac.update(signature_base.as_bytes());

    let sig = format!("v0={}", hex::encode(mac.finalize().into_bytes()));

    if sig != expt_sign {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Bot {
    pub bot_id: String,
    pub user_id: String,
}

pub async fn get_bot_id(token: &str) -> anyhow::Result<Bot> {
    let result = reqwest::Client::new()
        .post("https://slack.com/api/auth.test")
        .bearer_auth(token)
        .send()
        .await?;

    let bot: Bot = result.json().await?;
    Ok(bot)
}
