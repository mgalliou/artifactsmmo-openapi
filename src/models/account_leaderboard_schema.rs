/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountLeaderboardSchema {
    /// Position in the leaderboard.
    #[serde(rename = "position")]
    pub position: i32,
    /// Account name.
    #[serde(rename = "account")]
    pub account: String,
    /// Member status.
    #[serde(rename = "status")]
    pub status: models::AccountStatus,
    /// Achievements points.
    #[serde(rename = "achievements_points")]
    pub achievements_points: i32,
    /// Gold in the account.
    #[serde(rename = "gold")]
    pub gold: i32,
}

impl AccountLeaderboardSchema {
    pub fn new(position: i32, account: String, status: models::AccountStatus, achievements_points: i32, gold: i32) -> AccountLeaderboardSchema {
        AccountLeaderboardSchema {
            position,
            account,
            status,
            achievements_points,
            gold,
        }
    }
}

