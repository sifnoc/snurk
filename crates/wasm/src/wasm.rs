use console_error_panic_hook;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

use rand::thread_rng;

use snurk_engine::prove;

pub fn prove_inner(inputs: &str, zkey_file: &[u8], r1cs_file: &[u8], wasm_file: &[u8]) -> String {
    console_error_panic_hook::set_once();

    let inputs: Vec<(String, Vec<BigUint>)> = serde_json::from_str(inputs).unwrap();

    let (proof, public_inputs) = prove(inputs, zkey_file, r1cs_file, wasm_file);

    let proof_json = proof.clone().into_json();

    proof_json.to_string()
}