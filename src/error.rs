use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    //forward original err msg
    #[error("{0}")]
    Std(#[from] StdError),
    //log custom err msg
    #[error("Unauthorized - only {owner} can call function")]
    Unauthorized { owner: String },
}
