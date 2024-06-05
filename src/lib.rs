use users::Me;
use utils::create_client;

mod auth;
pub mod users;
mod utils;

const API: &str = "https://kick.com/api/v1";

pub enum KickAuthResult {
    Success,
    OTPRequired,
    TFARequired,
    InvalidCredentials,
    UnknownError,
}

pub enum KickAuthOTPResult {
    Success,
    InvalidOTP,
    UnknownError,
    NoLogged,
}

pub struct Kick {
    client: reqwest::Client,
    cached_email: Option<String>,
    cached_password: Option<String>,
    token: Option<String>,
}

impl Kick {
    pub fn new() -> Self {
        let client = create_client();
        Self {
            client,
            token: None,
            cached_email: None,
            cached_password: None,
        }
    }

    pub fn with_token(token: String) -> Self {
        let client = create_client();
        Self {
            client,
            token: Some(token),
            cached_email: None,
            cached_password: None,
        }
    }

    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    pub fn is_logged_in(&self) -> bool {
        self.token.is_some()
    }

    pub async fn login(&mut self, email: String, password: String) -> KickAuthResult {
        let result = auth::login(&self.client, email.clone(), password.clone(), None)
            .await
            .unwrap();
        let otp = result.otp_required.unwrap_or(false);
        let tfa = result.tfa_required.unwrap_or(false);

        if otp {
            self.cached_email = Some(email);
            self.cached_password = Some(password);
            return KickAuthResult::OTPRequired;
        } else if tfa {
            self.cached_email = Some(email);
            self.cached_password = Some(password);
            return KickAuthResult::TFARequired;
        } else if result.token.is_some() {
            self.token = Some(result.token.unwrap());
            self.cached_email = None;
            self.cached_password = None;
            return KickAuthResult::Success;
        } else {
            return KickAuthResult::InvalidCredentials;
        }
    }

    pub async fn login_otp(&mut self, otp: String) -> KickAuthOTPResult {
        if self.cached_email.is_none() || self.cached_password.is_none() {
            return KickAuthOTPResult::NoLogged;
        }

        let email = self.cached_email.clone().unwrap();
        let password = self.cached_password.clone().unwrap();

        let result = auth::login(&self.client, email, password, Some(otp))
            .await
            .unwrap();
        if result.token.is_some() {
            self.token = Some(result.token.unwrap());
            self.cached_email = None;
            self.cached_password = None;
            return KickAuthOTPResult::Success;
        } else {
            return KickAuthOTPResult::InvalidOTP;
        }
    }

    pub async fn get_me(&self) -> Option<Me> {
        let url = format!("{}/user", API);
        let res = self
            .client
            .get(url)
            .header(
                "Authorization",
                format!("Bearer {}", self.token.clone().unwrap()),
            )
            .send()
            .await
            .unwrap();
        let me = res.json::<Me>().await.unwrap();
        Some(me)
    }
}
