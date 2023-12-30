use cosmwasm_std::Coin;
use cw_storage_plus::Item;

//accessor to state object
pub const COUNTER: Item<u64> = Item::new("counter");

// minimal funds to be sent
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minmal_donation");
