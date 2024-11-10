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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DropRateSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Chance rate. (1/rate)
    #[serde(rename = "rate")]
    pub rate: i32,
    /// Minimum quantity.
    #[serde(rename = "min_quantity")]
    pub min_quantity: i32,
    /// Maximum quantity.
    #[serde(rename = "max_quantity")]
    pub max_quantity: i32,
}

impl DropRateSchema {
    pub fn new(code: String, rate: i32, min_quantity: i32, max_quantity: i32) -> DropRateSchema {
        DropRateSchema {
            code,
            rate,
            min_quantity,
            max_quantity,
        }
    }
}

