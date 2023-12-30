use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::{
    msg::InstantiateMsg,
    state::{COUNTER, MINIMAL_DONATION, OWNER},
};

pub fn instantiate(deps: DepsMut, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;
    MINIMAL_DONATION.save(deps.storage, &msg.minimal_donation)?;
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}
// use crate::msg::Val ueResp;
pub mod query {
    use crate::{msg::ValueResp, state::COUNTER};
    use cosmwasm_std::{Deps, StdResult};

    //query handler
    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

pub mod exec {
    use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult};

    use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};
    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // Cleaner way to update state (for future)
        // COUNTER.update(deps.storage, |counter| -> StdResult<_> { Ok(counter + 1) })?;

        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;
        let mut value = COUNTER.load(deps.storage)?;

        if minimal_donation.amount.is_zero()
            || info.funds.iter().any(|coin| {
                coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
            })
        {
            value += 1;
            COUNTER.save(deps.storage, &value)?;
        }

        let resp = Response::new()
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", value.to_string());

        Ok(resp)
    }

    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;

        if info.sender != owner {
            return Err(StdError::generic_err("not owner"));
        }

        //get funds from blockchain
        let funds = deps.querier.query_all_balances(&env.contract.address)?;

        //send bank msg to blockchain
        // to transfer funds without from one addr to other with executing any messages
        //sender of bank msg is not tx.origin it is address(this)
        let bank_msg = BankMsg::Send {
            to_address: owner.to_string(),
            amount: funds,
        };

        let res = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());

        Ok(res)
    }
}
