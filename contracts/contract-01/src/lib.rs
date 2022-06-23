use common::JsUint;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, json_types::U64, near_bindgen, AccountId, PanicOnDefault};

pub mod init;
pub mod non_wasm;
pub mod version;

/// A counter example.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Counter {
    /// Value that is tracked by the counter.
    pub value: U64,
}

#[near_bindgen]
impl Counter {
    /// Increments the internal counter value.
    ///
    /// #### Return
    ///
    /// Has no return.
    pub fn increment(&mut self) {
        self.value.0 += 1;
    }

    /// Decrements the internal counter value.
    ///
    /// #### Return
    ///
    /// Has no return.
    pub fn decrement(&mut self) {
        self.value.0 -= 1;
    }

    /// Gets the internal counter value.
    ///
    /// #### Return
    ///
    /// Returns the counter value.
    pub fn get(&self) -> U64 {
        self.value
    }

    /// Gets the internal counter value.
    ///
    /// #### Return
    ///
    /// Returns the counter value, if it's in range.
    pub fn get_as_integer(&self) -> JsUint {
        JsUint::new(self.value.0)
    }

    /// Sets the internal counter value.
    ///
    /// #### Return
    ///
    /// Has no return.
    pub fn set(
        &mut self,
        /// The new value of the counter.
        new_value: U64,
    ) {
        self.value = new_value;
    }

    /// Logs the internal counter value.
    ///
    /// #### Return
    ///
    /// Has no return.
    pub fn log_value(
        &self,
        /// Optionally also logs a mention to some user!
        mention: Option<AccountId>,
    ) {
        let msg = match mention {
            Some(mention) => format!("yo {}, you know what? {}", mention, self.value.0),
            None => format!("{}", self.value.0),
        };
        env::log_str(&msg)
    }
}
