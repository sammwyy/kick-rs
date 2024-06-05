use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

pub fn create_client() -> reqwest::Client {
    let cookie_store = CookieStore::new(None);
    let cookie_store = CookieStoreMutex::new(cookie_store);
    let cookie_store = Arc::new(cookie_store);

    let mut headers = HeaderMap::new();

    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert(
        "Accept-Language",
        "ar,en-US;q=0.7,en;q=0.3".parse().unwrap(),
    );
    headers.insert("Alt-Used", "kick.com".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/109.0"
            .parse()
            .unwrap(),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()
        .unwrap()
}
