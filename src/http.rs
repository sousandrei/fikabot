use std::env;

use mongodb::{Client, Database};
use serde::{Deserialize, Serialize};
use tracing::error;
use warp::{hyper::StatusCode, Filter};

use crate::{db, db::Channel};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackCommandBody {
    user_id: String,
    user_name: String,
    command: String,
    channel_id: String,
    channel_name: String,
}

pub async fn start() -> anyhow::Result<()> {
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not present on environment");

    let client = Client::with_uri_str(&mongo_url).await?;
    let db = client.database("fika");

    // 404
    let not_found = warp::path::end().map(|| "Hello, World at root!");

    // K8s health
    let metrics = warp::path!("metrics").map(|| StatusCode::OK);
    let healthcheck = warp::path!("healthchecks").map(|| StatusCode::OK);

    // POST /commands
    let commands = warp::path!("commands")
        .and(warp::post())
        .and(with_db(db))
        .and(warp::body::form())
        .and_then(handle_commands);

    let routes = commands
        .or(metrics)
        .or(healthcheck)
        .or(not_found)
        .with(warp::log("fika::api"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}

fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn handle_commands(
    db: Database,
    body: SlackCommandBody,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match body.command.as_str() {
        "/fika_join" => join_command(db, body).await,
        "/fika_leave" => leave_command(db, body).await,
        _ => {
            let res = warp::reply::with_status(
                warp::reply::html("Command not found"),
                StatusCode::NOT_FOUND,
            );

            Ok(Box::new(res))
        }
    }
}

async fn join_command(
    db: Database,
    body: SlackCommandBody,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let SlackCommandBody {
        channel_id,
        channel_name,
        ..
    } = body;

    let channel = Channel {
        channel_id,
        channel_name,
    };

    let message = match db::add_channel(db, channel).await {
        Ok(_) => "You just started the Fika roullete on this channel! :doughnut:",
        Err(e) => {
            error!("Error adding channel: {}", e);
            "There was an error trying to start the fika roullete here. Try again soon :thinking_face:"
        }
    };

    let res = warp::reply::with_status(warp::reply::html(message), StatusCode::OK);
    Ok(Box::new(res))
}

async fn leave_command(
    db: Database,
    body: SlackCommandBody,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let SlackCommandBody {
        channel_id,
        channel_name,
        ..
    } = body;

    let channel = Channel {
        channel_id,
        channel_name,
    };

    let message = match db::del_channel(db, channel).await {
        Ok(_) => "Sad to see you leave :cry:",
        Err(e) => {
            error!("Error deleting user: {}", e);
            "There was an error trying to disable the bot here. Try again soon :thinking_face:"
        }
    };

    let res = warp::reply::with_status(warp::reply::html(message), StatusCode::OK);
    Ok(Box::new(res))
}
