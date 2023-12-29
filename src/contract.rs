use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::state::COUNTER;

pub fn instantiate(deps: DepsMut) -> StdResult<Response> {
    COUNTER.save(deps.storage, &0)?; //? is err handler
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

    use crate::state::COUNTER;
    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // Cleaner way to update state (for future)
        // COUNTER.update(deps.storage, |counter| -> StdResult<_> { Ok(counter + 1) })?;

        let value = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &value)?;

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("countr", value.to_string());

        Ok(resp)
    }
}
