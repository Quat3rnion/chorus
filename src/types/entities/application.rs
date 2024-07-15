// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::{IntegrationApplication, PermissionFlags, Shared};
use crate::types::utils::Snowflake;
use crate::types::{Team, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// # Reference
/// See <https://discord.com/developers/docs/resources/application#application-resource>
pub struct Application {
    pub id: Snowflake,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub r#type: Option<ApplicationType>,
    pub hook: bool,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub verify_key: String,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub bot: Option<Shared<User>>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub owner: Shared<User>,
    pub flags: ApplicationFlags,
    #[cfg(feature = "sqlx")]
    pub redirect_uris: Option<sqlx::types::Json<Vec<String>>>,
    #[cfg(not(feature = "sqlx"))]
    pub redirect_uris: Option<Vec<String>>,
    pub rpc_application_state: i64,
    pub store_application_state: i64,
    pub verification_state: i64,
    pub interactions_endpoint_url: Option<String>,
    pub integration_public: bool,
    pub integration_require_code_grant: bool,
    pub discoverability_state: i64,
    pub discovery_eligibility_flags: i64,
    #[cfg(feature = "sqlx")]
    pub tags: Option<sqlx::types::Json<Vec<String>>>,
    #[cfg(not(feature = "sqlx"))]
    pub tags: Option<Vec<String>>,
    pub cover_image: Option<String>,
    #[cfg(feature = "sqlx")]
    pub install_params: Option<sqlx::types::Json<InstallParams>>,
    #[cfg(not(feature = "sqlx"))]
    pub install_params: Option<Shared<InstallParams>>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub team: Option<Team>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: "".to_string(),
            icon: None,
            description: None,
            summary: None,
            r#type: None,
            hook: true,
            bot_public: true,
            bot_require_code_grant: false,
            verify_key: "".to_string(),
            bot: Default::default(),
            owner: Default::default(),
            flags: ApplicationFlags::empty(),
            redirect_uris: None,
            rpc_application_state: 0,
            store_application_state: 1,
            verification_state: 1,
            interactions_endpoint_url: None,
            integration_public: true,
            integration_require_code_grant: false,
            discoverability_state: 1,
            discovery_eligibility_flags: 2240,
            tags: None,
            cover_image: None,
            install_params: None,
            terms_of_service_url: None,
            privacy_policy_url: None,
            team: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// # Reference
/// See <https://discord.com/developers/docs/resources/application#install-params-object>
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: PermissionFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://discord.com/developers/docs/resources/application#application-object-application-flags>
    pub struct ApplicationFlags: u64 {
        /// This application can create managed emoji
        const MANAGED_EMOJI = 1 << 2;
        /// This embedded application can use in-app purchases
        const EMBEDDED_IAP = 1 << 3;
        /// This application can create group DMs without limit
        const GROUP_DM_CREATE = 1 << 4;
        /// Indicates if an app uses the Auto Moderation API
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;
        /// Intent required for bots in 100 or more servers to receive presence_update events
        const GATEWAY_PRESENCE = 1 << 12;
        /// Intent required for bots in under 100 servers to receive presence_update events, found on the Bot page in your app's settings on discord.com
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;
        /// Intent required for bots in 100 or more servers to receive member-related events like guild_member_add.
        /// See the list of member-related events under GUILD_MEMBERS
        const GATEWAY_GUILD_MEMBERS = 1 << 14;
        /// Intent required for bots in under 100 servers to receive member-related events like guild_member_add, found on the Bot page in your app's settings on discord.com.
        /// See the list of member-related events under GUILD_MEMBERS
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
        /// Indicates unusual growth of an app that prevents verification
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
        /// Indicates if an app is embedded within the Discord client (currently unavailable publicly)
        const EMBEDDED = 1 << 17;
        /// Intent required for bots in 100 or more servers to receive message content
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;
        /// Intent required for bots in under 100 servers to receive message content, found on the Bot page in your app's settings on discord.com
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;
        /// Indicates if an app has registered slash commands
        const APPLICATION_COMMAND_BADGE = 1 << 23;
        /// This application has had at least one global application command used in the last 30 days
        const ACTIVE = 1 << 24;
        /// This application can use IFrames within modals
        const IFRAME_MODAL = 1 << 26;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// # Reference
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-object>
pub struct ApplicationCommand {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub name: String,
    pub description: String,
    pub options: Vec<Shared<ApplicationCommandOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Reference
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure>
pub struct ApplicationCommandOption {
    pub r#type: ApplicationCommandOptionType,
    pub name: String,
    pub description: String,
    pub required: bool,
    pub choices: Vec<ApplicationCommandOptionChoice>,
    pub options: Shared<Vec<ApplicationCommandOption>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(i32)]
/// # Reference
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-types>
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    /// Any integer between -2^53 and 2^53
    Integer = 4,
    Boolean = 5,
    User = 6,
    /// Includes all channel types + categories
    Channel = 7,
    Role = 8,
    /// Includes users and roles
    Mentionable = 9,
    /// Any double between -2^53 and 2^53
    Number = 10,
    Attachment = 11,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionData {
    pub id: Snowflake,
    pub name: String,
    pub options: Vec<Shared<ApplicationCommandInteractionDataOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    pub name: String,
    pub value: Value,
    pub options: Vec<Shared<ApplicationCommandInteractionDataOption>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-guild-application-command-permissions-structure>
pub struct GuildApplicationCommandPermissions {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub guild_id: Snowflake,
    pub permissions: Vec<Shared<ApplicationCommandPermission>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permissions-structure>
pub struct ApplicationCommandPermission {
    pub id: Snowflake,
    #[serde(rename = "type")]
    pub permission_type: ApplicationCommandPermissionType,
    /// true to allow, false, to disallow
    pub permission: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
/// See <https://discord.com/developers/docs/interactions/application-commands#application-command-permissions-object-application-command-permission-type>
pub enum ApplicationCommandPermissionType {
    #[default]
    Role = 1,
    User = 2,
    Channel = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// See <https://docs.discord.sex/resources/application#application-type>
pub enum ApplicationType {
    #[default]
    Game = 1,
    Music = 2,
    TicketedEvents = 3,
    CreatorMonetization = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#application-interactions-version>
pub enum ApplicationInteractionsVersion {
    #[default]
    Version1 = 1,
    Version2 = 2
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#explicit-content-filter-level>
pub enum ApplicationExplicitContentFilterLevel {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#application-verification-state>
pub enum ApplicationVerificationState {
    Ineligible = 1,
    #[default]
    Unsubmitted = 2,
    Submitted = 3,
    Succeeded = 4,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#store-application-state>
pub enum StoreApplicationState {
    #[default]
    None = 1,
    Paid = 2,
    Submitted = 3,
    Approved = 4,
    Rejected = 5
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#rpc-application-state>
pub enum RPCApplicationState {
    #[default]
    Disabled = 0,
    Unsubmitted = 1,
    Submitted = 2,
    Approved = 3,
    Rejected = 4
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#application-discoverability-state>
pub enum ApplicationDiscoveryState {
    Ineligible = 1,
    #[default]
    NotDiscoverable = 2,
    Discoverable = 3,
    Featureable = 4,
    Blocked = 5
}


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.sex/resources/application#application-discovery-eligibility-flags>
    pub struct ApplicationDiscoverEligibilityFlags: u64 {
        /// This application is verified
        const VERIFIED = 1 << 0;
        /// This application has at least one tag set
        const TAG = 1 << 1;
        /// This application has a description
        const DESCRIPTION = 1 << 2;
        /// This application has terms of service set
        const TERMS_OF_SERVICE = 1 << 3;
        /// This application has a privacy policy set
        const PRIVACY_POLICY = 1 << 4;
        /// This application has custom install URL or install parameters
        const INSTALL_PARAMS = 1 << 5;
        /// This application's name is safe for work
        const SAFE_NAME = 1 << 6;
        /// This application's description is safe for work
        const SAFE_DESCRIPTION = 1 << 7;
        /// This application has the message content intent approved or utilizes application commands
        const APPROVED_COMMANDS = 1 << 8;
        /// This application has a support guild set
        const SUPPORT_GUILD = 1 << 9;
        /// This application's commands are safe for work
        const SAFE_COMMANDS = 1 << 10;
        /// This application's owner has MFA enabled
        const MFA = 1 << 11;
        /// This application's long description is safe for work
        const SAFE_DIRECTORY_OVERVIEW = 1 << 12;
        /// This application has at least one supported locale set
        const SUPPORTED_LOCALES = 1 << 13;
        /// This application's directory short description is safe for work
        const SAFE_SHORT_DESCRIPTION = 1 << 14;
        /// This application's role connections metadata is safe for work
        const SAFE_ROLE_CONNECTIONS = 1 << 15;
        /// This application has met all the above criteria and is eligible to for discovery
        const ELIGIBLE = 1 << 16;
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#application-monetization-state>
pub enum ApplicationMonetizationState {
    #[default]
    None = 1,
    Enabled = 2,
    Blocked = 3,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, chorus_macros::SerdeBitFlags)]
    #[cfg_attr(feature = "sqlx", derive(chorus_macros::SqlxBitFlags))]
    /// # Reference
    /// See <https://docs.discord.sex/resources/application#application-monetization-eligibility-flags>
    pub struct ApplicationMonetizationEligibilityFlags: u64 {
        /// This application is verified
        const VERIFIED = 1 << 0;
        /// This application is owned by a team
        const HAS_TEAM = 1 << 1;
        /// This application has the message content intent approved or utilizes application commands
        const APPROVED_COMMANDS = 1 << 2;
        /// This application has terms of service set
        const TERMS_OF_SERVICE = 1 << 3;
        /// This application has a privacy policy set
        const PRIVACY_POLICY = 1 << 4;
        /// This application's name is safe for work
        const SAFE_NAME = 1 << 5;
        /// This application's description is safe for work
        const SAFE_DESCRIPTION = 1 << 6;
        /// This application's role connections metadata is safe for work
        const SAFE_ROLE_CONNECTIONS = 1 << 7;
        /// The user is the owner of the team that owns the application
        const USER_IS_TEAM_OWNER = 1 << 8;
        /// This application is not quarantined
        const NOT_QUARANTINED = 1 << 9;
        /// The user's locale is supported by monetization
        const USER_LOCALE_SUPPORTED = 1 << 10;
        /// The user is old enough to use monetization
        const USER_AGE_SUPPORTED = 1 << 11;
        /// The user has a date of birth is defined on their account
        const USER_DATE_OF_BIRTH_DEFINED = 1 << 12;
        /// The user has MFA enabled
        const USER_MFA_ENABLED = 1 << 13;
        /// The user's email is verified
        const USER_EMAIL_VERIFIED = 1 << 14;
        /// All members of the team that owns the application have verified emails
        const TEAM_MEMBERS_EMAIL_VERIFIED = 1 << 15;
        /// All members of the team that owns the application have MFA enabled
        const TEAM_MEMBERS_MFA_ENABLED = 1 << 16;
        /// This application has no issues blocking monetization
        const NO_BLOCKING_ISSUES = 1 << 17;
        /// The team has a valid payout status
        const VALID_PAYOUT_STATUS = 1 << 18;
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#application-executable-object>
pub struct ApplicationExecutableObject {
    pub os: String,
    pub name: String,
    pub is_launcher: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#application-sku-structure>
pub struct ApplicationSKUObject {
    pub id: String,
    pub sku: String,
    pub distributor: DistributorType
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
/// # Reference
/// See <https://docs.discord.sex/resources/application#distributor-type>
pub enum DistributorType {
    #[default]
    Discord,
    Steam,
    Twitch,
    Uplay,
    BattleNet,
    Origin,
    Gog,
    EpicGames,
    GooglePlay,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
/// # Reference
/// See <https://docs.discord.sex/resources/application#application-integration-type>
pub enum ApplicationIntegrationType {
    #[default]
    GuildInstall = 0,
    UserInstall = 1
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationProxyConfig {
    pub url_map: Vec<ProxyMap>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ProxyMap {
    pub prefix: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct EmbeddedActivityConfig {
    pub application_id: Option<Snowflake>,
    pub activity_preview_video_asset_id: Option<Snowflake>,
    pub supported_platforms: Option<Vec<EmbeddedActivityPlatformType>>,
    pub default_orientation_lock_state: OrientationLockState,
    pub tablet_default_orientation_lock_state: OrientationLockState,
    pub requires_age_gate: bool,
    pub client_platform_config: HashMap<String, EmbeddedActivityPlatformConfig>,
    pub shelf_rank: u32,
    pub has_csp_exception: bool,
    pub displays_advertisements: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub enum EmbeddedActivityPlatformType {
    #[default]
    Web,
    Android,
    Ios
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
pub enum OrientationLockState {
    #[default]
    Unlocked = 1,
    Portrait = 2,
    Landscape = 3
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct EmbeddedActivityPlatformConfig {
    pub label_type: ActivityLabelType,
    pub label_until: Option<DateTime<Utc>>,
    pub release_phase: ReleasePhase,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
pub enum ActivityLabelType {
    #[default]
    None = 0,
    New = 1,
    Updated = 2,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
pub enum ReleasePhase {
    #[default]
    InDevelopment,
    ActivitiesTeam,
    EmployeeRelease,
    SoftLaunch,
    GlobalLaunch
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ApplicationAsset {
    pub id: String,
    pub asset_type: AssetType,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
pub enum AssetType {
    #[default]
    One = 1,
    Two = 2
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ApplicationRoleConnection {
    pub platform_name: Option<String>,
    pub platform_username: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub application: Option<IntegrationApplication>,

}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ApplicationRoleConnectionMetadata {
    pub metadata_type: ApplicationRoleConnectionMetadataType,
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub name_localizations: HashMap<String, String>,
    pub description: String,
    #[serde(default)]
    pub description_localizations: HashMap<String, String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
pub enum ApplicationRoleConnectionMetadataType {
    #[default]
    IntegerLessThanOrEqual = 1,
    IntegerGreaterThanOrEqual = 2,
    IntegerEqual = 3,
    IntegerNotEqual = 4,
    DatetimeLessThanOrEqual = 5,
    DatetimeGreaterThanOrEqual = 6,
    BooleanEqual = 7,
    BooleanNotEqual = 8,
}