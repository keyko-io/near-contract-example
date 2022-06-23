use crate::Counter;
use near_sdk::{json_types::U64, near_bindgen};

#[cfg(not(target_arch = "wasm32"))]
use crate::CounterContract;

#[near_bindgen]
impl Counter {
    /// Creates a new instance. `value` starts at zero.
    #[init]
    pub fn new() -> Self {
        Counter { value: 0.into() }
    }

    /// Creates a new instance.
    #[init]
    pub fn new_with(
        /// Initial value.
        value: U64
    ) -> Self {
        Counter { value }
    }
}
