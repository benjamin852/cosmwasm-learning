use cosmwasm_std::Coin;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub minimal_donation: Coin,
}

//query msgs deserialize to this type
//enum since different queryMsgs will have different value
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Value {},
}

//struct since no need to distringuinsh between responses
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u64,
}

//used to update state
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecMsg {
    Donate {},
    Withdraw {},
}
