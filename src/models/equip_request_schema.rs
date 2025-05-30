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
pub struct EquipRequestSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Item slot.
    #[serde(rename = "slot")]
    pub slot: models::ItemSlot,
    /// Item details.
    #[serde(rename = "item")]
    pub item: Box<models::ItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl EquipRequestSchema {
    pub fn new(cooldown: models::CooldownSchema, slot: models::ItemSlot, item: models::ItemSchema, character: models::CharacterSchema) -> EquipRequestSchema {
        EquipRequestSchema {
            cooldown: Box::new(cooldown),
            slot,
            item: Box::new(item),
            character: Box::new(character),
        }
    }
}

