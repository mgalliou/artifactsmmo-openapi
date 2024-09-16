/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeTransactionListSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Transaction details.
    #[serde(rename = "transaction")]
    pub transaction: Box<models::GeTransactionSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl GeTransactionListSchema {
    pub fn new(cooldown: models::CooldownSchema, transaction: models::GeTransactionSchema, character: models::CharacterSchema) -> GeTransactionListSchema {
        GeTransactionListSchema {
            cooldown: Box::new(cooldown),
            transaction: Box::new(transaction),
            character: Box::new(character),
        }
    }
}

