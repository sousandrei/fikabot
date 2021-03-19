use hyper::{
    service::{make_service_fn, service_fn},
    Request, Server,
};
use hyper::{Body, Response};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::{
    signal::unix::{signal, SignalKind},
    task::JoinHandle,
};

use crate::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SlackBody {
    token: String,
    team_id: String,
    team_domain: String,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    text: String,
    api_app_id: String,
    is_enterprise_install: String,
    response_url: String,
    trigger_id: String,
}

pub fn start() -> JoinHandle<Result<(), Error>> {
    tokio::spawn(async move {
        let service =
            make_service_fn(|_connection| async { Ok::<_, Error>(service_fn(handle_request)) });

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
        let server = Server::bind(&addr).serve(service);

        let (tx, mut rx) = mpsc::channel(1);

        let graceful = server.with_graceful_shutdown(async {
            rx.recv().await;
            println!("Shutting down hyper server");
        });

        listen_for_signal(tx.clone(), SignalKind::interrupt());
        listen_for_signal(tx.clone(), SignalKind::terminate());

        println!("HTTP Online: {}", addr);

        graceful.await?;

        Ok(())
    })
}

fn listen_for_signal(tx: tokio::sync::mpsc::Sender<SignalKind>, kind: SignalKind) {
    tokio::task::spawn(async move {
        let mut stream =
            signal(kind).unwrap_or_else(|_| panic!("Error opening signal stream [{:?}]", kind));

        stream.recv().await;
        println!("Termination signal received");

        tx.send(kind).await.unwrap();
    });
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Validate request
    // If its right and all

    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

    let response = match serde_qs::from_bytes::<SlackBody>(&body_bytes) {
        Ok(body) => {
            println!("{:#?}", body);

            Response::builder().body(Body::from("all good"))?
        }
        Err(e) => {
            println!("{:?}", e);

            Response::builder().body(Body::from("not good"))?
        }
    };

    Ok(response)
}
