use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{hyper::StatusCode, Filter, Rejection, Reply};

use crate::{db, User};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackCommandBody {
    user_id: String,
    user_name: String,
    command: String,
}

pub async fn start() {
    // POST /commands
    let commands = warp::path!("commands")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(handle_commands);

    let routes = commands.recover(handle_rejection);

    println!("HTTP serving on port 8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    println!("err {:#?}", err);

    let res = warp::reply::with_status(warp::reply::html("Error"), StatusCode::OK);
    Ok(res)
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
