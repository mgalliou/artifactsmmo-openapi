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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillInfoSchema {
    /// The amount of xp gained.
    #[serde(rename = "xp")]
    pub xp: i32,
    /// Objects received.
    #[serde(rename = "items")]
    pub items: Vec<models::DropSchema>,
}

impl SkillInfoSchema {
    pub fn new(xp: i32, items: Vec<models::DropSchema>) -> SkillInfoSchema {
        SkillInfoSchema {
            xp,
            items,
        }
    }
}

