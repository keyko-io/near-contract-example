use common::{
    sim::contract_ext::{Call, ViewCall},
    JsUint,
};
use contract_01::non_wasm::methods::*;
pub use near_sdk::{
    json_types::{Base64VecU8, U128, U64},
    serde_json::json,
};
use near_sdk_sim::{init_simulator, UserAccount};
use near_units::parse_near;

pub mod utils;

use utils::Counter;

pub fn init() -> (
    UserAccount,
    (Counter),
    (UserAccount, UserAccount, UserAccount),
) {
    let root = init_simulator(None);

    let counter = utils::init(&root, None);

    let alice = root.create_user("alice".parse().unwrap(), parse_near!("10 kN"));
    let bob = root.create_user("bob".parse().unwrap(), parse_near!("10 kN"));
    let carol = root.create_user("carol".parse().unwrap(), parse_near!("10 kN"));

    (root, (counter), (alice, bob, carol))
}

#[test]
pub fn basic_tes() {
    let (ref root, (ref counter), (ref alice, ref bob, ref _carol)) = init();

    let _res = counter.increment().call(alice, None, None);
}
