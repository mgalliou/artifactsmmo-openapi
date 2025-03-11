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
pub struct InventorySlot {
    /// Inventory slot identifier.
    #[serde(rename = "slot")]
    pub slot: i32,
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Quantity in the slot.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl InventorySlot {
    pub fn new(slot: i32, code: String, quantity: i32) -> InventorySlot {
        InventorySlot {
            slot,
            code,
            quantity,
        }
    }
}

