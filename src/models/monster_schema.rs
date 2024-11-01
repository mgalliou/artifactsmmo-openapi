/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 2.3
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterSchema {
    /// Name of the monster.
    #[serde(rename = "name")]
    pub name: String,
    /// The code of the monster. This is the monster's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// Monster level.
    #[serde(rename = "level")]
    pub level: i32,
    /// Monster hit points.
    #[serde(rename = "hp")]
    pub hp: i32,
    /// Monster fire attack.
    #[serde(rename = "attack_fire")]
    pub attack_fire: i32,
    /// Monster earth attack.
    #[serde(rename = "attack_earth")]
    pub attack_earth: i32,
    /// Monster water attack.
    #[serde(rename = "attack_water")]
    pub attack_water: i32,
    /// Monster air attack.
    #[serde(rename = "attack_air")]
    pub attack_air: i32,
    /// Monster % fire resistance.
    #[serde(rename = "res_fire")]
    pub res_fire: i32,
    /// Monster % earth resistance.
    #[serde(rename = "res_earth")]
    pub res_earth: i32,
    /// Monster % water resistance.
    #[serde(rename = "res_water")]
    pub res_water: i32,
    /// Monster % air resistance.
    #[serde(rename = "res_air")]
    pub res_air: i32,
    /// Monster minimum gold drop. 
    #[serde(rename = "min_gold")]
    pub min_gold: i32,
    /// Monster maximum gold drop. 
    #[serde(rename = "max_gold")]
    pub max_gold: i32,
    /// Monster drops. This is a list of items that the monster drops after killing the monster. 
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropRateSchema>,
}

impl MonsterSchema {
    pub fn new(name: String, code: String, level: i32, hp: i32, attack_fire: i32, attack_earth: i32, attack_water: i32, attack_air: i32, res_fire: i32, res_earth: i32, res_water: i32, res_air: i32, min_gold: i32, max_gold: i32, drops: Vec<models::DropRateSchema>) -> MonsterSchema {
        MonsterSchema {
            name,
            code,
            level,
            hp,
            attack_fire,
            attack_earth,
            attack_water,
            attack_air,
            res_fire,
            res_earth,
            res_water,
            res_air,
            min_gold,
            max_gold,
            drops,
        }
    }
}

