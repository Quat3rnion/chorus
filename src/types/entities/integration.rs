// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Shared, entities::{Application, User}, utils::Snowflake, ApplicationType, ApplicationSKUObject};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See <https://discord.com/developers/docs/resources/guild#integration-object-integration-structure>
pub struct Integration {
    pub id: Snowflake,
    pub name: String,
    #[serde(rename = "type")]
    pub integration_type: IntegrationType,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enabled_emoticons: Option<bool>,
    pub expire_behaviour: Option<u8>,
    pub expire_grace_period: Option<u16>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Option<Shared<User>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub account: IntegrationAccount,
    pub synced_at: Option<DateTime<Utc>>,
    pub subscriber_count: Option<f64>,
    pub revoked: Option<bool>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub application: Option<Shared<Application>>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
/// See <https://discord.com/developers/docs/resources/guild#integration-account-object-integration-account-structure>
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(rename_all = "snake_case"))]
pub enum IntegrationType {
    #[default]
    Twitch,
    Youtube,
    Discord,
    GuildSubscription,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct IntegrationApplication {
    pub id: Snowflake,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub splash: Option<String>,
    pub application_type: Option<ApplicationType>,
    pub primary_sku_id: Option<Snowflake>,
    pub bot: Option<Shared<User>>, // TODO: should be a partial user
    pub deeplink_uri: Option<String>,
    #[serde(default)]
    pub third_party_skus: Vec<ApplicationSKUObject>,
    pub role_connections_verification_url: Option<String>,
}