use crate::*;
use wasm_bindgen::prelude::*;

const ZKEY: &[u8] = include_bytes!("/tmp/authorisationconstruction.00_temp.zkey");
const R1CS: &[u8] = include_bytes!("/home/jmart/git/Keyring/keyring-circuits/dist/circuits/AuthorisationConstruction/AuthorisationConstruction.r1cs"); 
const WASM: &[u8] = include_bytes!("/home/jmart/git/Keyring/keyring-circuits/dist/circuits/AuthorisationConstruction/AuthorisationConstruction.wasm");

#[wasm_bindgen]
pub fn prove(inputs: &str) -> String {
    prove_inner(inputs, ZKEY, R1CS, WASM)
}
