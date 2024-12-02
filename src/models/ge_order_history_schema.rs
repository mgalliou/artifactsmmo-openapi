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
pub struct GeOrderHistorySchema {
    /// Order id.
    #[serde(rename = "order_id")]
    pub order_id: String,
    /// Seller account name.
    #[serde(rename = "seller")]
    pub seller: String,
    /// Buyer account name.
    #[serde(rename = "buyer")]
    pub buyer: String,
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// Item price per unit.
    #[serde(rename = "price")]
    pub price: i32,
    /// Sale datetime.
    #[serde(rename = "sold_at")]
    pub sold_at: String,
}

impl GeOrderHistorySchema {
    pub fn new(order_id: String, seller: String, buyer: String, code: String, quantity: i32, price: i32, sold_at: String) -> GeOrderHistorySchema {
        GeOrderHistorySchema {
            order_id,
            seller,
            buyer,
            code,
            quantity,
            price,
            sold_at,
        }
    }
}

