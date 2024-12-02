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
pub struct ItemEffectSchema {
    /// Effect name.
    #[serde(rename = "name")]
    pub name: String,
    /// Effect value.
    #[serde(rename = "value")]
    pub value: i32,
}

impl ItemEffectSchema {
    pub fn new(name: String, value: i32) -> ItemEffectSchema {
        ItemEffectSchema {
            name,
            value,
        }
    }
}

