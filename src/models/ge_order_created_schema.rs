/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 4.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeOrderCreatedSchema {
    /// Order id.
    #[serde(rename = "id")]
    pub id: String,
    /// Order created at.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// Item price per unit.
    #[serde(rename = "price")]
    pub price: i32,
    /// Total price.
    #[serde(rename = "total_price")]
    pub total_price: i32,
    /// Order creation tax (3%, minimum 1)
    #[serde(rename = "tax")]
    pub tax: i32,
}

impl GeOrderCreatedSchema {
    pub fn new(id: String, created_at: String, code: String, quantity: i32, price: i32, total_price: i32, tax: i32) -> GeOrderCreatedSchema {
        GeOrderCreatedSchema {
            id,
            created_at,
            code,
            quantity,
            price,
            total_price,
            tax,
        }
    }
}

