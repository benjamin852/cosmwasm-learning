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
