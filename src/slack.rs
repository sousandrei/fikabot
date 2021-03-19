use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
struct SlackMessage {
    channel: String,
    text: String,
}

pub async fn send_message(user: &User, msg: &str) {
    println!("person {:#?} msg {:#?}", user.user_name, msg);

    // let User { user_id, .. } = user;

    // let msg = SlackMessage {
    //     channel: format!("@{}", user_id),
    //     text: "<@userID> eita".to_string(),
    // };

    // let token = "?";

    // let res: serde_json::Value = reqwest::Client::new()
    //     .post("https://slack.com/api/chat.postMessage")
    //     .bearer_auth(token)
    //     .json(&msg)
    //     .send()
    //     .await
    //     .unwrap()
    //     .json()
    //     .await
    //     .unwrap();

    // println!("{:#?}", res);
}
