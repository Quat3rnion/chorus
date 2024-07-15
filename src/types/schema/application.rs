use serde::{Deserialize, Serialize};
use crate::types::{ApplicationFlags, ApplicationType, Snowflake};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ApplicationCreateSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub application_type: Option<ApplicationType>,
    pub team_id: Option<Snowflake>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<ApplicationFlags>,
    pub guild_id: Option<Snowflake>,
    #[serde(default)]
    pub redirect_uris: Vec<String>,
    pub deeplink_uri: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct ApplicationModifySchema {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub interactions_endpoint_url: Option<String>,
    pub max_participants: Option<u32>,
    pub privacy_policy_url: Option<String>,
    pub role_connections_verification_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub terms_of_service_url: Option<String>,
    pub bot_public: Option<bool>,
    pub bot_require_code_grant: Option<bool>,
    pub flags: Option<ApplicationFlags>
}