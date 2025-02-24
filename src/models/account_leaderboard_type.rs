/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 3.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountLeaderboardType {
    #[serde(rename = "achievements_points")]
    AchievementsPoints,

}

impl std::fmt::Display for AccountLeaderboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AchievementsPoints => write!(f, "achievements_points"),
        }
    }
}

impl Default for AccountLeaderboardType {
    fn default() -> AccountLeaderboardType {
        Self::AchievementsPoints
    }
}

