use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Router,
};
use http_body_util::BodyExt;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use entity::{prelude::*, *};

use crate::{
    algos::{fika, song},
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

pub async fn start(config: &Config, db: &DatabaseConnection) -> anyhow::Result<()> {
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let commands_router = Router::new()
        .route("/commands", post(parse_commands))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(config.clone()))
                .layer(Extension(db.clone()))
                .layer(middleware::from_fn(slack_auth)),
        );

    let http_router = Router::new()
        .route("/start_fika", post(start_fika))
        .route("/start_song", post(start_song))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(config.clone()))
                .layer(Extension(db.clone()))
                .layer(middleware::from_fn(token_auth)),
        );

    let app = Router::new()
        .merge(commands_router)
        .merge(http_router)
        .route("/ping", get(ping))
        .layer(tracing_layer);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port.unwrap_or(8080)));

    tracing::info!("server started on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await?;

    // TODO: health
    // let metrics = warp::path!("metrics").map(|| StatusCode::OK);
    // let healthcheck = warp::path!("healthchecks").map(|| StatusCode::OK);

    Ok(())
}

async fn ping() -> &'static str {
    "pong!"
}

async fn token_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    let config: &Config = parts.extensions.get().ok_or_else(|| {
        tracing::error!("no config on auth middleware");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let headers = parts.headers.clone();

    let header_token = headers.get("x-token");
    if header_token.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if header_token.is_none() || *header_token.unwrap() != config.webhook_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}

async fn slack_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    let config: &Config = parts.extensions.get().ok_or_else(|| {
        tracing::error!("no config on auth middleware");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let headers = parts.headers.clone();

    let timestamp = headers.get("X-Slack-Request-Timestamp");
    if timestamp.is_none() {
        tracing::error!("no timestamp on slack auth");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let timestamp = timestamp.unwrap().to_str().map_err(|_| {
        tracing::error!("timestamp on slack auth is not a string");
        StatusCode::UNAUTHORIZED
    })?;

    let signature = headers.get("X-Slack-Signature");
    if signature.is_none() {
        tracing::error!("no signature on slack auth");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let signature = signature.unwrap().to_str().map_err(|_| {
        tracing::error!("signature on slack auth is not a string");
        StatusCode::UNAUTHORIZED
    })?;

    let bytes = body
        .collect()
        .await
        .map_err(|_| {
            tracing::error!("cannot collect body");
            StatusCode::UNAUTHORIZED
        })?
        .to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();

    if let Err(e) = slack::verify_slack(
        &config.slack_signing_secret,
        signature,
        timestamp,
        &body_str,
    ) {
        tracing::error!("Slack auth header error: {}", e);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let req = Request::from_parts(parts, Body::from(bytes));
    Ok(next.run(req).await)
}

async fn start_fika(
    Extension(config): Extension<Config>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    if let Err(e) = fika::matchmake(&config, &db).await {
        tracing::error!("fika error: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

async fn start_song(
    Extension(config): Extension<Config>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    if let Err(e) = song::matchmake(&config, &db).await {
        tracing::error!("song error: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

async fn parse_commands(
    Extension(config): Extension<Config>,
    Extension(db): Extension<DatabaseConnection>,
    mut req: Request,
) -> impl IntoResponse {
    let body = req.body_mut();
    let bytes = body
        .collect()
        .await
        .map_err(|_| {
            tracing::error!("cannot collect body");
            StatusCode::UNAUTHORIZED
        })
        .unwrap()
        .to_bytes();

    let data: SlackCommandBody = serde_qs::from_bytes(&bytes).unwrap();

    match data.command.as_str() {
        "/fika_now" => now_fika(&config.slack_token, data).await,
        "/song_now" => now_song(&config, &db).await,
        "/fika_start" => start_command(&db, data).await,
        "/fika_stop" => stop_command(&db, data).await,
        "/fika_song" => song_command(&db, data).await,
        _ => "Command not found",
    }
}

async fn now_song(config: &Config, db: &DatabaseConnection) -> &'static str {
    if let Err(e) = song::matchmake(config, db).await {
        tracing::error!("{}", e);
        return "Error starting song matchmaking";
    }

    "Song matchmaking started!"
}

async fn now_fika(token: &str, body: SlackCommandBody) -> &'static str {
    let SlackCommandBody {
        channel_id,
        channel_name,
        ..
    } = body;

    if channel_name == "general" {
        return "Fika is not allowed in general :/";
    }

    let channel = channel::Model {
        id: channel_id,
        name: channel_name,
    };

    if let Err(e) = fika::matchmake_channel(token, &channel).await {
        tracing::error!("{:#?}", e);
        return "Error running matchmaking :/";
    }

    "Fika started!"
}

async fn start_command(db: &DatabaseConnection, body: SlackCommandBody) -> &'static str {
    let SlackCommandBody {
        channel_id,
        channel_name,
        ..
    } = body;

    if channel_name == "general" {
        return "Fika is not allowed in general :/";
    }

    if channel_name == "directmessage" {
        return "Fika is only allowed in channels :D";
    }

    let channel = channel::Model {
        id: channel_id.clone(),
        name: channel_name,
    }
    .into_active_model();

    match Channel::find_by_id(channel_id).one(db).await {
        Ok(c) => {
            if c.is_some() {
                return "Fika is already running in this channel :D";
            } else if let Err(e) = channel.insert(db).await {
                tracing::error!("Error saving channel: {}", e);
                return "There was an error trying to start the fika roullete here. Try again soon :thinking_face:";
            }

            "You just started the Fika roullete on this channel! :doughnut:"
        }
        Err(e) => {
            tracing::error!("Error finding channel: {}", e);
            "There was an error trying to start the fika roullete here. Try again soon :thinking_face:"
        }
    }
}

async fn stop_command(db: &DatabaseConnection, body: SlackCommandBody) -> &'static str {
    let SlackCommandBody { channel_id, .. } = body;

    if let Err(e) = Channel::delete_by_id(channel_id).exec(db).await {
        tracing::error!("Error deleting channel: {}", e);
    }

    "Sad to see you stop :cry:"
}

async fn song_command(db: &DatabaseConnection, body: SlackCommandBody) -> &'static str {
    let SlackCommandBody {
        user_id,
        user_name,
        text,
        ..
    } = body;

    let song = match validate_url(text) {
        Some(url) => url,
        None => return "This url is not valid :/",
    };

    let user = user::Model {
        id: user_id.clone(),
        name: user_name.clone(),
        song: Some(song),
    };

    match User::find_by_id(user_id).one(db).await {
        Ok(u) => {
            if u.is_some() {
                let mut old_user = u.unwrap().into_active_model();

                old_user.name = Set(user.name);
                old_user.song = Set(user.song);

                if let Err(e) = old_user.update(db).await {
                    tracing::error!("Error updating user: {}", e);
                    return "There was an error trying to save your song. Try again soon :thinking_face:";
                }
            } else if let Err(e) = user.into_active_model().insert(db).await {
                tracing::error!("Error saving user: {}", e);
                return "There was an error trying to save your song. Try again soon :thinking_face:";
            }

            "Your song is saved for this week! :partyparrot:"
        }
        Err(e) => {
            tracing::error!("Error finding user: {}", e);
            "There was an error trying to save your song. Try again soon :thinking_face:"
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::http::validate_url;

    #[test]
    fn valid_url() {
        assert_eq!(
            validate_url("https://music.youtube.com/watch?v=pA_v6zYJDAI&feature=share".into()),
            Some("music.youtube.com/watch?v=pA_v6zYJDAI&feature=share".into())
        );
        assert_eq!(
            validate_url("https://www.youtube.com/watch?v=AV1mu0rsHxc".into()),
            Some("youtube.com/watch?v=AV1mu0rsHxc".into())
        );
        assert_eq!(
            validate_url("https://youtu.be/AV1mu0rsHxc".into()),
            Some("youtu.be/AV1mu0rsHxc".into())
        );
        assert_eq!(
            validate_url("http://music.youtube.com/watch?v=pA_v6zYJDAI&feature=share".into()),
            Some("music.youtube.com/watch?v=pA_v6zYJDAI&feature=share".into())
        );
        assert_eq!(
            validate_url("http://www.youtube.com/watch?v=AV1mu0rsHxc".into()),
            Some("youtube.com/watch?v=AV1mu0rsHxc".into())
        );
        assert_eq!(
            validate_url("http://youtu.be/AV1mu0rsHxc".into()),
            Some("youtu.be/AV1mu0rsHxc".into())
        );
        assert_eq!(
            validate_url(
                "https://open.spotify.com/track/3BGj9WOKMyl2ZlkK8IoKhq?si=8771121b200647e5".into()
            ),
            Some("open.spotify.com/track/3BGj9WOKMyl2ZlkK8IoKhq?si=8771121b200647e5".into())
        );
        assert_eq!(
            validate_url("youtu.be/AV1mu0rsHxc".into()),
            Some("youtu.be/AV1mu0rsHxc".into())
        );
        assert_eq!(validate_url("e/AV1mu0rsHxc".into()), None);
        assert_eq!(validate_url("barracuda".into()), None);
        assert_eq!(validate_url("u.be/AV1mu0rsHxc".into()), None);
        assert_eq!(validate_url("http://a.be/AV1mu0rsHxc".into()), None);
    }
}
