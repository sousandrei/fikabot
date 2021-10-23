use serde::{Deserialize, Serialize};
use tide::{log::error, Request};

use crate::db::{channel::Channel, user::User};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackCommandBody {
    user_id: String,
    user_name: String,
    command: String,
    channel_id: String,
    channel_name: String,
    text: String,
}

pub async fn start() -> anyhow::Result<()> {
    let mut app = tide::Server::new();

    app.at("/commands").post(parse_commands);

    // TODO: health
    // let metrics = warp::path!("metrics").map(|| StatusCode::OK);
    // let healthcheck = warp::path!("healthchecks").map(|| StatusCode::OK);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn parse_commands(mut req: Request<()>) -> tide::Result {
    let body: SlackCommandBody = req.body_form().await?;

    match body.command.as_str() {
        "/fika_start" => start_command(body).await,
        "/fika_stop" => stop_command(body).await,
        "/fika_song" => song_command(body).await,
        _ => Ok("Command not found".into()),
    }
}

async fn start_command(body: SlackCommandBody) -> tide::Result {
    let SlackCommandBody {
        channel_id,
        channel_name,
        ..
    } = body;

    let channel = Channel {
        channel_id,
        channel_name,
    };

    let message = match channel.save().await {
        Ok(_) => "You just started the Fika roullete on this channel! :doughnut:",
        Err(e) => {
            error!("Error adding channel: {}", e);
            "There was an error trying to start the fika roullete here. Try again soon :thinking_face:"
        }
    };

    Ok(message.into())
}

async fn stop_command(body: SlackCommandBody) -> tide::Result {
    let SlackCommandBody { channel_id, .. } = body;

    let message = match Channel::delete(&channel_id).await {
        Ok(_) => "Sad to see you stop :cry:",
        Err(e) => {
            error!("Error deleting user: {}", e);
            "There was an error trying to disable the bot here. Try again soon :thinking_face:"
        }
    };

    Ok(message.into())
}

async fn song_command(body: SlackCommandBody) -> tide::Result {
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

    let message = match user.save().await {
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
