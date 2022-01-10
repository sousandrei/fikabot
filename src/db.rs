use anyhow::bail;
use goauth::{
    auth::{JwtClaims, Token},
    credentials::Credentials,
    fetcher::TokenFetcher,
    scopes::Scope,
};
use serde::{Deserialize, Serialize};
use smpl_jwt::Jwt;
use std::{env, str::FromStr};
use time::Duration;

pub mod channel;
pub mod user;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Response {
    values: Vec<Vec<String>>,
}

const TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v4/token";

async fn get_token() -> anyhow::Result<Token> {
    let credentials = env::var("CREDENTIALS").expect("CREDENTIALS not present on environment");
    let account_email =
        env::var("ACCOUNT_EMAIL").expect("ACCOUNT_EMAIL not present on environment");

    let credentials = Credentials::from_str(&credentials).unwrap();
    let claims = JwtClaims::new(
        account_email,
        &Scope::SpreadSheets,
        TOKEN_URL.to_owned(),
        None,
        None,
    );
    let jwt = Jwt::new(claims, credentials.rsa_key().unwrap(), None);

    let fetcher = TokenFetcher::new(jwt, credentials, Duration::new(1, 0));

    match fetcher.fetch_token().await {
        Ok(token) => Ok(token),
        Err(e) => bail!("fail fetching token: {}", e),
    }
}

pub async fn get_values(sheet: &str) -> anyhow::Result<Response> {
    let sheets_id = env::var("SHEETS_ID").expect("SHEETS_ID not present on environment");

    let token = get_token().await?;

    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
        sheets_id, sheet
    );

    let result = reqwest::Client::new()
        .get(url)
        .bearer_auth(token.access_token())
        .send()
        .await?;

    let values: Response = result.json().await?;
    Ok(values)
}

pub async fn write_values(sheet: &str, values: &Response) -> anyhow::Result<()> {
    let sheets_id = env::var("SHEETS_ID").expect("SHEETS_ID not present on environment");

    let token = get_token().await?;

    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=USER_ENTERED",
        sheets_id, sheet
    );

    reqwest::Client::new()
        .put(url)
        .bearer_auth(token.access_token())
        .json(values)
        .send()
        .await?;

    Ok(())
}
