use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use msg::{ExecMsg, InstantiateMsg};

//contract logic. private so other cant manipulate
mod contract;

//msg module. public so others can interact
pub mod msg;

// mod for contract state
mod state;

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
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(_deps, _info, _msg)
}

//binary is a serialized response for the query (Ex. json data)
#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_json_binary(&contract::query::value(_deps)?),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> StdResult<Response> {
    use msg::ExecMsg::*;

    match msg {
        Donate {} => contract::exec::donate(deps, info),
        Withdraw {} => contract::exec::withdraw(deps, _env, info),
    }
}

//unit tests
#[cfg(test)]
mod test {
    use crate::msg::{QueryMsg, ValueResp};

    use super::*;
    use cosmwasm_std::{coins, Addr, Coin, Empty};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    const ATOM: &str = "atom";

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
                &InstantiateMsg {
                    minimal_donation: Coin::new(10, ATOM),
                },
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

    #[test]
    fn donate_with_funds() {
        let sender = Addr::unchecked("sender");

        //init atom balances for address
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, ATOM))
                .unwrap();
        });

        //register contract in blockchain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg {
                    minimal_donation: Coin::new(10, ATOM),
                },
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, ATOM),
        )
        .unwrap();

        // wrap converts app object to query wrapper obj.
        // Query wrapper obj can be used to query from chain
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });

        //check balance transfer occured
        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
        assert_eq!(
            app.wrap().query_all_balances(contract_addr).unwrap(),
            coins(10, ATOM)
        );
    }

    #[test]
    fn donate_without_funds() {
        let sender = Addr::unchecked("sender");

        //init atom balances for address
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, ATOM))
                .unwrap();
        });

        //register contract in blockchain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg {
                    minimal_donation: Coin::new(10, ATOM),
                },
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        app.execute_contract(sender, contract_addr.clone(), &ExecMsg::Donate {}, &[])
            .unwrap();

        // wrap converts app object to query wrapper obj.
        // Query wrapper obj can be used to query from chain
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }
    #[test]
    fn withdraw() {
        let owner = Addr::unchecked("owner");
        let sender = Addr::unchecked("sender");
        let sender2 = Addr::unchecked("sender-two");

        //init atom balances for address
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, ATOM))
                .unwrap();
            router
                .bank
                .init_balance(storage, &sender2, coins(5, ATOM))
                .unwrap();
        });

        //register contract in blockchain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    minimal_donation: Coin::new(10, ATOM),
                },
                &[],
                "Counting Contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, ATOM),
        )
        .unwrap();

        app.execute_contract(
            sender2.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(5, ATOM),
        )
        .unwrap();

        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &ExecMsg::Withdraw {},
            &[],
        )
        .unwrap();

        //check balance transfer occured
        assert_eq!(
            app.wrap().query_all_balances(owner).unwrap(),
            coins(15, ATOM)
        );
        assert_eq!(
            app.wrap().query_all_balances(contract_addr).unwrap(),
            vec![]
        );
        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
        assert_eq!(app.wrap().query_all_balances(sender2).unwrap(), vec![]);
    }
}
