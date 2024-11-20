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
pub struct GeCancelOrderSchema {
    /// Order id.
    #[serde(rename = "id")]
    pub id: String,
}

impl GeCancelOrderSchema {
    pub fn new(id: String) -> GeCancelOrderSchema {
        GeCancelOrderSchema {
            id,
        }
    }
}

