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
pub struct CharacterMovementDataSchema {
    /// Cooldown details
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Destination details.
    #[serde(rename = "destination")]
    pub destination: Box<models::MapSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl CharacterMovementDataSchema {
    pub fn new(cooldown: models::CooldownSchema, destination: models::MapSchema, character: models::CharacterSchema) -> CharacterMovementDataSchema {
        CharacterMovementDataSchema {
            cooldown: Box::new(cooldown),
            destination: Box::new(destination),
            character: Box::new(character),
        }
    }
}

