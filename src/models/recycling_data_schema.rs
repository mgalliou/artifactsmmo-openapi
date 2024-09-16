/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 1.6
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecyclingDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Craft details.
    #[serde(rename = "details")]
    pub details: Box<models::RecyclingItemsSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl RecyclingDataSchema {
    pub fn new(cooldown: models::CooldownSchema, details: models::RecyclingItemsSchema, character: models::CharacterSchema) -> RecyclingDataSchema {
        RecyclingDataSchema {
            cooldown: Box::new(cooldown),
            details: Box::new(details),
            character: Box::new(character),
        }
    }
}

