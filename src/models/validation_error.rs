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
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<models::ValidationErrorLocInner>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ValidationError {
    pub fn new(loc: Vec<models::ValidationErrorLocInner>, msg: String, r#type: String) -> ValidationError {
        ValidationError {
            loc,
            msg,
            r#type,
        }
    }
}

