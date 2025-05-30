/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 4.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CraftSkill {
    #[serde(rename = "weaponcrafting")]
    Weaponcrafting,
    #[serde(rename = "gearcrafting")]
    Gearcrafting,
    #[serde(rename = "jewelrycrafting")]
    Jewelrycrafting,
    #[serde(rename = "cooking")]
    Cooking,
    #[serde(rename = "woodcutting")]
    Woodcutting,
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "alchemy")]
    Alchemy,

}

impl std::fmt::Display for CraftSkill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Weaponcrafting => write!(f, "weaponcrafting"),
            Self::Gearcrafting => write!(f, "gearcrafting"),
            Self::Jewelrycrafting => write!(f, "jewelrycrafting"),
            Self::Cooking => write!(f, "cooking"),
            Self::Woodcutting => write!(f, "woodcutting"),
            Self::Mining => write!(f, "mining"),
            Self::Alchemy => write!(f, "alchemy"),
        }
    }
}

impl Default for CraftSkill {
    fn default() -> CraftSkill {
        Self::Weaponcrafting
    }
}

