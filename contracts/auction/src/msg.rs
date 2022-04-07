use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::OfferState;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateOffer{name:String,description:String,days_active:u32},
    PlaceBid{offer_owner:String},
    FinnishOffer{offer_owner:String},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    
    GetOffer {offer_owner:String},
    GetAllOffers ,
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferResponse {
    pub name: String,
    pub description: String,
    pub highest_bid: u64,
    pub deadline: u64,
    pub state:OfferState,
}
