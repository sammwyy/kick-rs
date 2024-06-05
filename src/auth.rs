use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

use reqwest::{Client, Error};

use crate::utils;

const API_TOKEN_PROVIDER: &str = "https://kick.com/kick-token-provider";
const API_LOGIN: &str = "https://kick.com/mobile/login";

// Serde.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthTokenResult {
    // enabled: bool,
    name_field_name: String,
    // unrandomized_name_field_name: String,
    valid_from_field_name: String,
    encrypted_valid_from: String,
}

async fn get_token() -> Result<AuthTokenResult, Error> {
    let client = utils::create_client();
    let res = client.get(API_TOKEN_PROVIDER).send().await?;
    res.json::<AuthTokenResult>().await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResult {
    #[serde(rename = "2fa_required")]
    pub tfa_required: Option<bool>,
    pub otp_required: Option<bool>,
    pub token: Option<String>,
}

pub async fn login(
    client: &Client,
    email: String,
    password: String,
    otp: Option<String>,
) -> Result<AuthResult, Error> {
    let tokens = get_token().await?;
    let mut body: HashMap<&str, Value> = HashMap::new();

    body.insert("email", Value::String(email));
    body.insert("password", Value::String(password));
    if otp.is_some() {
        let otp = otp.unwrap();
        body.insert("one_time_password", Value::String(otp));
    }

    body.insert(&tokens.name_field_name, Value::String("".to_string()));
    body.insert(
        &tokens.valid_from_field_name,
        Value::String(tokens.encrypted_valid_from),
    );
    body.insert("isMobileRequest", Value::Bool(true));

    let json = json!(body);

    let res = client.post(API_LOGIN).json(&json).send().await?;
    let body = res.json::<AuthResult>().await?;
    Ok(body)
}
