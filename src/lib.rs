use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};

//contract logic. private so other cant manipulate
mod contract;

//msg module. public so others can interact
pub mod msg;

// mod for contract state
mod state;

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

// akin to constructor
/**
 * param @deps: Obj that allows us to interact with blockchain (ex. use storage, query other contracts). Mut means you can alter state
 * param @env: Info about current blockchain state (ex. block height, timestamp)
 * param @_info: Info about message that was sent (ex. who is sender, when it was sent)
 * param @msg: the msg itself
 * return @response  Alias for rust *result* type with different error type
 */
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    contract::instantiate(_deps)
}

//binary is a serialized response for the query (Ex. json data)
#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_json_binary(&contract::query::value(_deps)?),
    }
}

//unit tests
#[cfg(test)]
mod test {
    use crate::msg::{QueryMsg, ValueResp};

    use super::*;
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        //app obj is blockchain simulator
        let mut app = App::default();

        //register contract in blockchain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &QueryMsg::Value {},
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        // wrap converts app object to query wrapper obj.
        // Query wrapper obj can be used to query from chain
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }
}
