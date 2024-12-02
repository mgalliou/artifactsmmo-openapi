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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CraftSchema {
    /// Skill required to craft the item.
    #[serde(rename = "skill", skip_serializing_if = "Option::is_none")]
    pub skill: Option<models::CraftSkill>,
    /// The skill level required to craft the item.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// List of items required to craft the item.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::SimpleItemSchema>>,
    /// Quantity of items crafted.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl CraftSchema {
    pub fn new() -> CraftSchema {
        CraftSchema {
            skill: None,
            level: None,
            items: None,
            quantity: None,
        }
    }
}

