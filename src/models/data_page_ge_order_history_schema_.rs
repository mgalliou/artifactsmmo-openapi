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
pub struct DataPageGeOrderHistorySchema {
    #[serde(rename = "data")]
    pub data: Vec<models::GeOrderHistorySchema>,
    #[serde(rename = "total", deserialize_with = "Option::deserialize")]
    pub total: Option<i32>,
    #[serde(rename = "page", deserialize_with = "Option::deserialize")]
    pub page: Option<i32>,
    #[serde(rename = "size", deserialize_with = "Option::deserialize")]
    pub size: Option<i32>,
    #[serde(rename = "pages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pages: Option<Option<i32>>,
}

impl DataPageGeOrderHistorySchema {
    pub fn new(data: Vec<models::GeOrderHistorySchema>, total: Option<i32>, page: Option<i32>, size: Option<i32>) -> DataPageGeOrderHistorySchema {
        DataPageGeOrderHistorySchema {
            data,
            total,
            page,
            size,
            pages: None,
        }
    }
}

