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
pub struct DeleteItemSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Item details.
    #[serde(rename = "item")]
    pub item: Box<models::SimpleItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl DeleteItemSchema {
    pub fn new(cooldown: models::CooldownSchema, item: models::SimpleItemSchema, character: models::CharacterSchema) -> DeleteItemSchema {
        DeleteItemSchema {
            cooldown: Box::new(cooldown),
            item: Box::new(item),
            character: Box::new(character),
        }
    }
}

