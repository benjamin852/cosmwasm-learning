use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

//accessor to state object
pub const COUNTER: Item<u64> = Item::new("counter");

// minimal funds to be sent
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minmal_donation");

pub const OWNER: Item<Addr> = Item::new("owner");
