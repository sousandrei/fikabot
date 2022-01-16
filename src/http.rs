use serde::{Deserialize, Serialize};
use tide::{log::error, Request, Response, StatusCode};

use crate::{
    algos::{fika, song},
    db::{channel::Channel, user::User},
    slack, Config,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackCommandBody {
    user_id: String,
    user_name: String,
    command: String,
    channel_id: String,
    channel_name: String,
    text: String,
}

pub async fn start(config: &Config) -> anyhow::Result<()> {
    let mut app = tide::with_state(config.clone());

    app.at("/commands").post(parse_commands);
    app.at("/start_fika").post(start_fika);
    app.at("/start_song").post(start_song);
    app.at("/ping").get(ping);

    // TODO: health
    // let metrics = warp::path!("metrics").map(|| StatusCode::OK);
    // let healthcheck = warp::path!("healthchecks").map(|| StatusCode::OK);

    let port = config.port.clone().unwrap_or_else(|| "8080".into());

    app.listen(format!("0.0.0.0:{port}")).await?;

    Ok(())
}

async fn ping(_: Request<Config>) -> tide::Result {
    Ok("pong!".into())
}

fn validate_webhook(req: Request<Config>) -> Result<(), Response> {
    let config = req.state();

    let header_token = req.header("x-token");

    if header_token.is_none() || *header_token.unwrap() != config.webhook_token {
        return Err(Response::new(StatusCode::Unauthorized));
    }

    Ok(())
}

async fn start_song(req: Request<Config>) -> tide::Result {
    let config = req.state().clone();

    if let Err(e) = validate_webhook(req) {
        return Ok(e);
    }

    song::matchmake(&config).await?;
    Ok(Response::new(StatusCode::Ok))
}

async fn start_fika(req: Request<Config>) -> tide::Result {
    let config = req.state().clone();

    if let Err(e) = validate_webhook(req) {
        return Ok(e);
    }

    fika::matchmake(&config).await?;
    Ok(Response::new(StatusCode::Ok))
}

async fn parse_commands(mut req: Request<Config>) -> tide::Result {
    let timestamp = req.header("X-Slack-Request-Timestamp").cloned().unwrap();
    let signature = req.header("X-Slack-Signature").cloned().unwrap();

    let body = req.body_string().await?;
    let config = req.state();

    if let Err(e) = slack::verify_slack(
        &config.slack_signing_secret,
        signature.as_str(),
        timestamp.as_str(),
        &body,
    ) {
        return Ok(Response::new(e));
    }

    let body: SlackCommandBody = serde_qs::from_str(&body)?;

    match body.command.as_str() {
        "/fika_now" => now_command(&config.slack_token, body).await,
        "/fika_start" => start_command(config, body).await,
        "/fika_stop" => stop_command(config, body).await,
        "/fika_song" => song_command(config, body).await,
        _ => Ok("Command not found".into()),
    }
}

async fn now_command(token: &str, body: SlackCommandBody) -> tide::Result {
    let SlackCommandBody {
        channel_id,
        channel_name,
        ..
    } = body;

    if channel_name == "general" {
        return Ok("Fika is not allowed in general :/".into());
    }

    let channel = Channel {
        channel_id,
        channel_name,
    };

    fika::matchmake_channel(token, &channel).await?;

    Ok("Fika started!".into())
}

async fn start_command(config: &crate::Config, body: SlackCommandBody) -> tide::Result {
    let SlackCommandBody {
        channel_id,
        channel_name,
        ..
    } = body;

    if channel_name == "general" {
        return Ok("Fika is not allowed in general :/".into());
    }

    if channel_name == "directmessage" {
        return Ok("Fika is only allowed in channels :D".into());
    }

    let channel = Channel {
        channel_id,
        channel_name,
    };

    let message = match channel.save(config).await {
        Ok(_) => "You just started the Fika roullete on this channel! :doughnut:",
        Err(e) => {
            error!("Error adding channel: {}", e);
            "There was an error trying to start the fika roullete here. Try again soon :thinking_face:"
        }
    };

    Ok(message.into())
}

async fn stop_command(config: &crate::Config, body: SlackCommandBody) -> tide::Result {
    let SlackCommandBody { channel_id, .. } = body;

    let message = match Channel::delete(config, &channel_id).await {
        Ok(_) => "Sad to see you stop :cry:",
        Err(e) => {
            error!("Error deleting user: {}", e);
            "There was an error trying to disable the bot here. Try again soon :thinking_face:"
        }
    };

    Ok(message.into())
}

async fn song_command(config: &crate::Config, body: SlackCommandBody) -> tide::Result {
    let SlackCommandBody {
        user_id,
        user_name,
        text,
        ..
    } = body;

    let song = match validate_url(text) {
        Some(url) => url,
        None => return Ok("This url is not valid :/".into()),
    };

    let user = User {
        user_id,
        user_name,
        song,
    };

    let message = match user.save(config).await {
        Ok(_) => "Your song is saved for this week! :partyparrot:",
        Err(e) => {
            error!("Error saving user: {}", e);
            "There was an error trying to save your song. Try again soon :thinking_face:"
        }
    };

    Ok(message.into())
}

const VALID_URLS: [&str; 5] = [
    "open.spotify.com",
    "youtube.com",
    "youtu.be",
    "music.youtube.com",
    "soundcloud.com",
];

fn validate_url(url: String) -> Option<String> {
    let url = url
        .replace("https://", "")
        .replace("http://", "")
        .replace("www.", "");

    for valid_url in VALID_URLS {
        if url.starts_with(valid_url) {
            return Some(url);
        }
    }

    None
}

// TODO: Free test cases
// dbg!(validate_url("https://music.youtube.com/watch?v=pA_v6zYJDAI&feature=share".into()));
// dbg!(validate_url("https://www.youtube.com/watch?v=AV1mu0rsHxc".into()));
// dbg!(validate_url("https://youtu.be/AV1mu0rsHxc".into()));

// dbg!(validate_url("http://music.youtube.com/watch?v=pA_v6zYJDAI&feature=share".into()));
// dbg!(validate_url("http://www.youtube.com/watch?v=AV1mu0rsHxc".into()));
// dbg!(validate_url("http://youtu.be/AV1mu0rsHxc".into()));

// dbg!(validate_url("https://open.spotify.com/track/3BGj9WOKMyl2ZlkK8IoKhq?si=8771121b200647e5".into()));

// dbg!(validate_url("youtu.be/AV1mu0rsHxc".into()));
// dbg!(validate_url("e/AV1mu0rsHxc".into()));
// dbg!(validate_url("barracuda".into()));
// dbg!(validate_url("u.be/AV1mu0rsHxc".into()));
// dbg!(validate_url("http://a.be/AV1mu0rsHxc".into()));
