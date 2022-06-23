pub use contract_01;

fn main() {
    let spec = contract_01::non_wasm::openapi();
    let spec_str = near_sdk::serde_json::to_string_pretty(&spec).unwrap();
    println!("{}", spec_str);
}
