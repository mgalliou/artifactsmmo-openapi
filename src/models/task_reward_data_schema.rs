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
pub struct TaskRewardDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Reward details.
    #[serde(rename = "reward")]
    pub reward: Box<models::TaskRewardSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl TaskRewardDataSchema {
    pub fn new(cooldown: models::CooldownSchema, reward: models::TaskRewardSchema, character: models::CharacterSchema) -> TaskRewardDataSchema {
        TaskRewardDataSchema {
            cooldown: Box::new(cooldown),
            reward: Box::new(reward),
            character: Box::new(character),
        }
    }
}

