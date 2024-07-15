// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::types::entities::User;
use crate::types::Snowflake;
use crate::types::Shared;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Team {
    pub icon: Option<String>,
    pub id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Snowflake,
    pub payout_account_status: Option<TeamPayoutAccountStatus>,
    pub stripe_connect_account_id: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TeamPayoutAccountStatus {
    #[default]
    Unsubmitted = 1,
    Pending = 2,
    ActionRequired = 3,
    Active = 4,
    Blocked = 5,
    Suspended = 6,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TeamMember {
    pub membership_state: MembershipState,
    pub team_id: Snowflake,
    #[cfg_attr(feature = "sqlx", sqlx(skip))]
    pub user: Shared<User>,
    pub role: TeamMemberRole,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[repr(u8)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MembershipState {
    #[default]
    Invited = 1,
    Accepted = 2,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
pub enum TeamMemberRole {
    Owner,
    Admin,
    Developer,
    #[default]
    ReadOnly
}