/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 3.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationSchema {
    /// The x coordinate of the destination.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y coordinate of the destination.
    #[serde(rename = "y")]
    pub y: i32,
}

impl DestinationSchema {
    pub fn new(x: i32, y: i32) -> DestinationSchema {
        DestinationSchema {
            x,
            y,
        }
    }
}

