use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::{
    msg::InstantiateMsg,
    state::{COUNTER, MINIMAL_DONATION},
};

pub fn instantiate(deps: DepsMut, msg: InstantiateMsg) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?;
    MINIMAL_DONATION.save(deps.storage, &msg.minimal_donation)?;
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
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

    use crate::state::{COUNTER, MINIMAL_DONATION};
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
}
