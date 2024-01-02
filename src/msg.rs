use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
    pub minimal_donation: Coin,
}

//query msgs deserialize to this type
//enum since different queryMsgs will have different value
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
}

//used to update state

#[cw_serde]
pub enum ExecMsg {
    Donate {},
    Reset {
        #[serde(default)]
        counter: u64,
    },
    Withdraw {},
    WithdrawTo {
        receiver: String,
        #[serde(default)]
        funds: Vec<Coin>,
    },
}
//struct since no need to distringuinsh between responses
#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}
