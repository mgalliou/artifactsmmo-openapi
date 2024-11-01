/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapSchema {
    /// Name of the map.
    #[serde(rename = "name")]
    pub name: String,
    /// Skin of the map.
    #[serde(rename = "skin")]
    pub skin: String,
    /// Position X of the map.
    #[serde(rename = "x")]
    pub x: i32,
    /// Position Y of the map.
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "content", deserialize_with = "Option::deserialize")]
    pub content: Option<Box<models::MapContentSchema>>,
}

impl MapSchema {
    pub fn new(name: String, skin: String, x: i32, y: i32, content: Option<models::MapContentSchema>) -> MapSchema {
        MapSchema {
            name,
            skin,
            x,
            y,
            content: if let Some(x) = content {Some(Box::new(x))} else {None},
        }
    }
}

