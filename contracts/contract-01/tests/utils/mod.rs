use common::sim::{contract_ext::Deploy, ContractExt};
use contract_01::CounterContract;
use near_sdk_sim::{ContractAccount, UserAccount};
use near_units::parse_near;

pub type Counter = ContractAccount<CounterContract>;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_WASM_BYTES => "../res/contract_01.wasm",
}

pub struct Here;
impl near_sdk::HasContract<Here> for Counter {
    type Contract = CounterContract;

    fn contract(&self) -> &Self::Contract {
        &self.contract
    }
}

pub fn init(
    root: &UserAccount,
    contract_id: impl Into<Option<String>>,
) -> Counter {
    use contract_01::non_wasm::methods::*;

    let contract_id = contract_id.into().unwrap_or_else(|| "counter".into());

    let counter = Counter::new().deploy(
        root,
        contract_id,
        &COUNTER_WASM_BYTES,
        None,
        parse_near!("0 N"),
    );
    counter.transfer_extra_deposit_to(root);
    counter
}
