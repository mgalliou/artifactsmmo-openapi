/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AchievementType {
    #[serde(rename = "combat_kill")]
    CombatKill,
    #[serde(rename = "combat_drop")]
    CombatDrop,
    #[serde(rename = "combat_level")]
    CombatLevel,
    #[serde(rename = "gathering")]
    Gathering,
    #[serde(rename = "crafting")]
    Crafting,
    #[serde(rename = "recycling")]
    Recycling,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "other")]
    Other,

}

impl std::fmt::Display for AchievementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CombatKill => write!(f, "combat_kill"),
            Self::CombatDrop => write!(f, "combat_drop"),
            Self::CombatLevel => write!(f, "combat_level"),
            Self::Gathering => write!(f, "gathering"),
            Self::Crafting => write!(f, "crafting"),
            Self::Recycling => write!(f, "recycling"),
            Self::Task => write!(f, "task"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl Default for AchievementType {
    fn default() -> AchievementType {
        Self::CombatKill
    }
}

