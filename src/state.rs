use cw_storage_plus::Item;

//accessor to state object
pub const COUNTER: Item<u64> = Item::new("counter");
