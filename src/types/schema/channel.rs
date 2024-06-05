// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;

use crate::types::{ChannelType, DefaultReaction, Error, entities::PermissionOverwrite, Snowflake};

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct ChannelCreateSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: Option<ChannelType>,
    pub topic: Option<String>,
    pub icon: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub parent_id: Option<Snowflake>,
    pub id: Option<Snowflake>,
    pub nsfw: Option<bool>,
    pub rtc_region: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_reaction_emoji: Option<String>,
    pub flags: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub struct ChannelModifySchema {
    pub name: Option<String>,
    pub channel_type: Option<ChannelType>,
    pub topic: Option<String>,
    pub icon: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub parent_id: Option<Snowflake>,
    pub nsfw: Option<bool>,
    pub rtc_region: Option<String>,
    pub default_auto_archive_duration: Option<i32>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub flags: Option<i32>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub video_quality_mode: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct GetChannelMessagesSchema {
    /// Between 1 and 100, defaults to 50.
    pub limit: Option<i32>,
    #[serde(flatten)]
    pub anchor: ChannelMessagesAnchor,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMessagesAnchor {
    Before(Snowflake),
    Around(Snowflake),
    After(Snowflake),
}

impl GetChannelMessagesSchema {
    pub fn before(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: ChannelMessagesAnchor::Before(anchor),
        }
    }

    pub fn around(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: ChannelMessagesAnchor::Around(anchor),
        }
    }

    pub fn after(anchor: Snowflake) -> Self {
        Self {
            limit: None,
            anchor: ChannelMessagesAnchor::After(anchor),
        }
    }

    /// Must be between 1 and 100
    pub fn limit(self, limit: i32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct CreateChannelInviteSchema {
    pub flags: Option<InviteFlags>,
    pub max_age: Option<u32>,
    pub max_uses: Option<u8>,
    pub temporary: Option<bool>,
    pub unique: Option<bool>,
    pub validate: Option<String>,
    pub target_type: Option<InviteTargetType>,
    pub target_user_id: Option<Snowflake>,
    pub target_application_id: Option<Snowflake>,
}

impl Default for CreateChannelInviteSchema {
    fn default() -> Self {
        Self {
            flags: None,
            max_age: Some(86400),
            max_uses: Some(0),
            temporary: Some(false),
            unique: Some(false),
            validate: None,
            target_type: None,
            target_user_id: None,
            target_application_id: None,
        }
    }
}

bitflags! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct InviteFlags: u64 {
        const GUEST = 1 << 0;
        const VIEWED = 1 << 1;
    }
}

impl Serialize for InviteFlags {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.bits().to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InviteFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct FlagsVisitor;

        impl<'de> Visitor<'de> for FlagsVisitor
        {
            type Value = InviteFlags;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a raw u64 value of flags")
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> where E: serde::de::Error {
                InviteFlags::from_bits(v).ok_or(serde::de::Error::custom(Error::InvalidFlags(v)))
            }
        }

        deserializer.deserialize_u64(FlagsVisitor)
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::MySql> for InviteFlags {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        u64::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::MySql> for InviteFlags {
    fn encode_by_ref(&self, buf: &mut <sqlx::MySql as sqlx::database::HasArguments<'q>>::ArgumentBuffer) -> sqlx::encode::IsNull {
        u64::encode_by_ref(&self.0.0, buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::MySql> for InviteFlags {
    fn decode(value: <sqlx::MySql as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let raw = u64::decode(value)?;

        Ok(Self::from_bits(raw).unwrap())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum InviteType {
    #[default]
    Guild = 0,
    GroupDm = 1,
    Friend = 2,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum InviteTargetType {
    #[default]
    Stream = 1,
    EmbeddedApplication = 2,
    RoleSubscriptions = 3,
    CreatorPage = 4,
}

/// See <https://discord-userdoccers.vercel.app/resources/channel#add-channel-recipient>
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct AddChannelRecipientSchema {
    pub access_token: Option<String>,
    pub nick: Option<String>,
}

/// See <https://discord-userdoccers.vercel.app/resources/channel#add-channel-recipient>
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
pub struct ModifyChannelPositionsSchema {
    pub id: Snowflake,
    pub position: Option<u32>,
    pub lock_permissions: Option<bool>,
    pub parent_id: Option<Snowflake>,
}
