use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Me {
    pub id: u32,
    pub username: String,
    pub bio: String,
    pub country: String,
    pub state: String,
    pub city: String,
    pub instagram: String,
    pub tiktok: String,
    pub youtube: String,
    pub twitter: String,
    pub discord: String,
    pub facebook: String,

    pub agreed_to_terms: bool,
    pub email_verified_at: Option<String>,
    pub enable_live_notifications: bool,
    pub enable_onscreen_live_notifications: bool,
    pub email_updated_at: Option<String>,

    #[serde(rename = "profilePic")]
    pub profile_pic: String,
    pub is_live: bool,
}
