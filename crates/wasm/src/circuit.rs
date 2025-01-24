use crate::*;
use wasm_bindgen::prelude::*;

const ZKEY: &[u8] = include_bytes!("/tmp/keccak256_256_test_final_temp.zkey");
const R1CS: &[u8] = include_bytes!("/home/jin/Projects/zkmopro/snurk/test-vectors/keccak256_256_test.r1cs"); 
const WASM: &[u8] = include_bytes!("/home/jin/Projects/zkmopro/snurk/test-vectors/keccak256_256_test.wasm");

#[wasm_bindgen]
pub fn prove(inputs: &str) -> String {
    prove_inner(inputs, ZKEY, R1CS, WASM)
}
