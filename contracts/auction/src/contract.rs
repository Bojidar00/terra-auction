#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,Order,CosmosMsg,BankMsg,Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{OfferResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, Offer,OFFERS, OfferState};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:auction";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateOffer{name,description,days_active} =>create_offer(deps,info,env,name,description,days_active),
        ExecuteMsg::PlaceBid{offer_owner}=>place_bid(deps, info, env, offer_owner),
        ExecuteMsg::FinnishOffer{offer_owner}=>finish_offer(deps,env, offer_owner),
    }
}
pub fn create_offer(deps: DepsMut, info: MessageInfo,env:Env,name:String,description:String,days_active:u32) ->Result<Response,ContractError>{
    let time = env.block.time;
    let millis = (time.seconds() * 1_000) + (time.nanos() / 1_000_000);
   
   OFFERS.update(deps.storage, info.sender.to_string(), | offer: Option<Offer>| -> Result<_, ContractError> {
          match offer {
            Some(_offer)=>{return Err(ContractError::LastOfferStillActive)},
            None =>{let new_offer = Offer{name:name,description:description,owner:info.sender , highest_bid:("".to_string(),0),bids:Vec::new(),deadline:(millis+((days_active as u64)*(60*60*24*1000))) as u64,state:OfferState::Active};Ok(new_offer)},
        } 
       
        
    })?;  
    




    Ok(Response::new().add_attribute("method", "create"))
}

pub fn place_bid(deps: DepsMut, info: MessageInfo,env:Env, owner:String)->Result<Response,ContractError>{
    let time = env.block.time;
    let millis = (time.seconds() * 1_000) + (time.nanos() / 1_000_000);

    let offer = OFFERS.load(deps.storage, owner.clone())?;
    if offer.deadline > millis{
    let (_account,bid)=offer.highest_bid;
    if info.funds[0].amount.u128()>bid as u128{
        OFFERS.update(deps.storage,owner, | offer: Option<Offer>| -> Result<_, ContractError> {
            match offer {
                Some(mut _offer)=>{
                    let (account,bid)=_offer.highest_bid.clone();
                    if account!=""{
                    _offer.bids.push((account, bid));}
                    _offer.highest_bid=(info.sender.to_string(),info.funds[0].amount.u128() as u64); 
                    Ok(_offer)},
                None=>{Err(ContractError::WrongOffer)}
            }
        
        })?;
    }else{
        return Err(ContractError::LessThanCurrentBid)
    }
    }else{
        return Err(ContractError::OfferClosed)
    }    

    Ok(Response::new())
}

pub fn finish_offer(deps:DepsMut,env:Env,offer_owner:String)->Result<Response,ContractError>{
    let offer = OFFERS.load(deps.storage, offer_owner.clone())?;

    let time = env.block.time;
    let millis = (time.seconds() * 1_000) + (time.nanos() / 1_000_000);
    if offer.state==OfferState::Active{
    if offer.deadline < millis{
    
    let mut msgs:Vec<CosmosMsg> = Vec::new();
    let (_account,bid)=offer.highest_bid;
    let amount =vec![ Coin::new((bid ) as u128, "uluna")];
    msgs.push(CosmosMsg::Bank(BankMsg::Send{to_address:offer.owner.to_string(),amount:amount}));

    for player in offer.bids{
        let (p_account,p_bid)=player;
        let p_amount =vec![ Coin::new((p_bid ) as u128, "uluna")];
        msgs.push(CosmosMsg::Bank(BankMsg::Send{to_address:p_account,amount:p_amount}));
    } 
    OFFERS.update(deps.storage,offer_owner, | offer: Option<Offer>| -> Result<_, ContractError> {
       let mut _offer=offer.unwrap();
       _offer.state=OfferState::Closed;
       Ok(_offer)
    
    })?;
    Ok(Response::new().add_messages(msgs))
    }else{
        return Err(ContractError::OfferActive)
    }
    }else{
        return Err(ContractError::OfferClosed)
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOffer{offer_owner}=>to_binary(&query_offer(deps,offer_owner)?),
        QueryMsg::GetAllOffers=>to_binary(&query_all_offers(deps)?),
    }
}


fn query_offer(deps: Deps, offer_owner:String) -> StdResult<OfferResponse> {
    let offer = OFFERS.load(deps.storage, offer_owner)?;
    let (_account,bid)=offer.highest_bid;
    Ok(OfferResponse {name: offer.name,description:offer.description,highest_bid:bid,deadline:offer.deadline,state: offer.state })
} 

fn query_all_offers(deps:Deps)->StdResult<Vec<OfferResponse>>{
    let mut offers=Vec::new();
    let all: StdResult<Vec<_>> = OFFERS
    .range(deps.storage, None, None, Order::Ascending)
    .collect();

    match all {
        Ok(_offers)=>{
            for o in _offers{
                let (_adr, offer) = o;
                let (_account,bid)=offer.highest_bid;
                offers.push(OfferResponse{

                    name: offer.name,
                    description: offer.description,
                    highest_bid: bid,
                    deadline:offer.deadline,
                    state:offer.state,
                });
            }
            },
        Err(_)=>{}

    };
    Ok(offers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

       
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

       
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::CreateOffer {name:"test".to_string(),description:"test offer".to_string(),days_active:1};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetAllOffers {}).unwrap();
        let value: Vec<OfferResponse> = from_binary(&res).unwrap();
        println!("{:?}",value);
       assert_eq!(1,value.len());
    }

    #[test]
    fn placebid() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

       
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::CreateOffer {name:"test".to_string(),description:"test offer".to_string(),days_active:1};
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let info2 = mock_info("anyone", &coins(2, "luna"));
        let msg = ExecuteMsg::PlaceBid {offer_owner:info.sender.to_string()};
        let _res = execute(deps.as_mut(), mock_env(), info2, msg).unwrap();



        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetAllOffers {}).unwrap();
        let value: Vec<OfferResponse> = from_binary(&res).unwrap();
       assert_eq!(1,value.len());
    }

    #[test]
    fn finnish_offer() {
        let mut deps = mock_dependencies(&[]);
        let mut env = mock_env();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

       
        let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::CreateOffer {name:"test".to_string(),description:"test offer".to_string(),days_active:1};
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let info2 = mock_info("anyone", &coins(2, "luna"));
        let msg = ExecuteMsg::PlaceBid {offer_owner:info.sender.to_string()};
        let _res = execute(deps.as_mut(), env.clone(), info2, msg).unwrap();

        let info3 = mock_info("anyone2", &coins(3, "luna"));
        let msg = ExecuteMsg::PlaceBid {offer_owner:info.sender.to_string()};
        let _res = execute(deps.as_mut(), env.clone(), info3, msg).unwrap();

        env.block.time = mock_env().block.time.plus_seconds(60*60*24);

        let info5 = mock_info("anyone2", &coins(0, "luna"));
        let msg = ExecuteMsg::FinnishOffer {offer_owner:info.sender.to_string()};
        let _res = execute(deps.as_mut(), env, info5, msg).unwrap();


        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOffer {offer_owner:info.sender.to_string()}).unwrap();
        let value: OfferResponse = from_binary(&res).unwrap();
       assert_eq!(OfferState::Closed,value.state);
    }  
} 
