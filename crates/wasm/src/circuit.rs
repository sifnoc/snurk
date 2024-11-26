use crate::*;
use wasm_bindgen::prelude::*;

const ZKEY: &[u8] = include_bytes!("/tmp/semaphore-32_temp.zkey");
const R1CS: &[u8] = include_bytes!("/home/jmart/git/TheFrozenFire/snurk/semaphore-32.r1cs"); 
const WASM: &[u8] = include_bytes!("/home/jmart/git/TheFrozenFire/snurk/crates/wasm/src/circuit.rs");

#[wasm_bindgen]
pub fn prove(inputs: &str) -> String {
    prove_inner(inputs, ZKEY, R1CS, WASM)
}
