/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json 
 *
 * The version of the OpenAPI document: 3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionType {
    #[serde(rename = "movement")]
    Movement,
    #[serde(rename = "fight")]
    Fight,
    #[serde(rename = "crafting")]
    Crafting,
    #[serde(rename = "gathering")]
    Gathering,
    #[serde(rename = "buy_ge")]
    BuyGe,
    #[serde(rename = "sell_ge")]
    SellGe,
    #[serde(rename = "cancel_ge")]
    CancelGe,
    #[serde(rename = "delete_item")]
    DeleteItem,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdraw")]
    Withdraw,
    #[serde(rename = "deposit_gold")]
    DepositGold,
    #[serde(rename = "withdraw_gold")]
    WithdrawGold,
    #[serde(rename = "equip")]
    Equip,
    #[serde(rename = "unequip")]
    Unequip,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "recycling")]
    Recycling,
    #[serde(rename = "rest")]
    Rest,
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "buy_bank_expansion")]
    BuyBankExpansion,

}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Movement => write!(f, "movement"),
            Self::Fight => write!(f, "fight"),
            Self::Crafting => write!(f, "crafting"),
            Self::Gathering => write!(f, "gathering"),
            Self::BuyGe => write!(f, "buy_ge"),
            Self::SellGe => write!(f, "sell_ge"),
            Self::CancelGe => write!(f, "cancel_ge"),
            Self::DeleteItem => write!(f, "delete_item"),
            Self::Deposit => write!(f, "deposit"),
            Self::Withdraw => write!(f, "withdraw"),
            Self::DepositGold => write!(f, "deposit_gold"),
            Self::WithdrawGold => write!(f, "withdraw_gold"),
            Self::Equip => write!(f, "equip"),
            Self::Unequip => write!(f, "unequip"),
            Self::Task => write!(f, "task"),
            Self::Recycling => write!(f, "recycling"),
            Self::Rest => write!(f, "rest"),
            Self::Use => write!(f, "use"),
            Self::BuyBankExpansion => write!(f, "buy_bank_expansion"),
        }
    }
}

impl Default for ActionType {
    fn default() -> ActionType {
        Self::Movement
    }
}

