use crate::Counter;
use common::version::{version_from_env, IVersion, Version};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CounterContract;

#[near_bindgen]
impl IVersion for Counter {
    /// Get versioning information.
    ///
    /// #### Return
    ///
    /// Returns the versioning info.
    fn version(&self) -> Version {
        version_from_env!()
    }
}
