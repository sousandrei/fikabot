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

    let user = User {
        user_id,
        user_name,
        song: text,
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
