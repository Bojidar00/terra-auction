use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_storage_plus::Map;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum OfferState {
    Active,
    Closed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub name: String,
    pub description: String,
    pub owner: Addr,
    pub highest_bid: (String,u64),
    pub bids:Vec<(String, u64)>,
    pub deadline: u64,
    pub state: OfferState,
}

pub const STATE: Item<State> = Item::new("state");
pub const OFFERS: Map<String, Offer> = Map::new("offers");
