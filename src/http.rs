use serde::{Deserialize, Serialize};
use warp::{hyper::StatusCode, Filter};

use crate::{db, User};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackCommandBody {
    user_id: String,
    user_name: String,
    command: String,
}

pub async fn start() {
    // 404
    let not_found = warp::path::end().map(|| "Hello, World at root!");

    // K8s health
    let metrics = warp::path!("metrics").map(|| StatusCode::OK);
    let healthcheck = warp::path!("healthchecks").map(|| StatusCode::OK);

    // POST /commands
    let commands = warp::path!("commands")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(handle_commands);

    let routes = commands
        .or(metrics)
        .or(healthcheck)
        .or(not_found)
        .with(warp::log("lunch::api"));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_commands(body: SlackCommandBody) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match body.command.as_str() {
        "/lunch_join" => join_command(body).await,
        "/lunch_leave" => leave_command(body).await,
        _ => {
            let res = warp::reply::with_status(
                warp::reply::html("Command not found"),
                StatusCode::NOT_FOUND,
            );

            Ok(Box::new(res))
        }
    }
}

async fn join_command(body: SlackCommandBody) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    // delete anyone with same id/workspace
    // create a user

    let SlackCommandBody {
        user_id, user_name, ..
    } = body;

    let user = User { user_id, user_name };

    db::add_user(user).await;

    let res = warp::reply::with_status(
        warp::reply::html("You just joined the lunch roullette! See you next monday!"),
        StatusCode::OK,
    );

    Ok(Box::new(res))
}

async fn leave_command(body: SlackCommandBody) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let SlackCommandBody {
        user_id, user_name, ..
    } = body;

    let user = User { user_id, user_name };

    db::del_user(user).await;

    let res =
        warp::reply::with_status(warp::reply::html("Sad to see you leave :c"), StatusCode::OK);

    Ok(Box::new(res))
}
