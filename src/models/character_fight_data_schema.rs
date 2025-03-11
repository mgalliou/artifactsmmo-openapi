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
pub struct CharacterFightDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Fight details.
    #[serde(rename = "fight")]
    pub fight: Box<models::FightSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl CharacterFightDataSchema {
    pub fn new(cooldown: models::CooldownSchema, fight: models::FightSchema, character: models::CharacterSchema) -> CharacterFightDataSchema {
        CharacterFightDataSchema {
            cooldown: Box::new(cooldown),
            fight: Box::new(fight),
            character: Box::new(character),
        }
    }
}

