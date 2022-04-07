use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("You cannot post another offer before your last one ends.")]
    LastOfferStillActive,
    #[error("Your bid should be more than the current highest bid.")]
    LessThanCurrentBid,
    #[error("This offer doesn't exist.")]
    WrongOffer,
    #[error("This offer is closed.")]
    OfferClosed,
    #[error("This offer is still active.")]
    OfferActive,
}
