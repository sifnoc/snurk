use ark_bn254::{Bn254, Fr as BnScalar};
use ark_circom::{CircomBuilder, CircomConfig, CircomReduction, WitnessCalculator, circom::{ R1CSFile, R1CS } };
use ark_groth16::{Groth16, ProvingKey, Proof};
use ark_serialize::CanonicalDeserialize;
use ark_snark::SNARK;

use num_bigint::BigUint;

use rand::thread_rng;
use wasmer::{Store, Module};

pub fn prove(
    witness_inputs: Vec<(String, Vec<BigUint>)>,
    zkey_file: &[u8],
    r1cs_file: &[u8],
    wasm_file: &[u8],
) -> (Proof<Bn254>, Vec<BnScalar>) {
    let mut rng = thread_rng();

    let mut store = Store::default();
    let witness_generator_module = Module::from_binary(&store, wasm_file).unwrap();

    let wtns = WitnessCalculator::from_module(&mut store, witness_generator_module).unwrap();

    let r1cs_reader = std::io::Cursor::new(r1cs_file);
    let r1cs_file = R1CSFile::<BnScalar>::new(r1cs_reader).unwrap();
    let r1cs = R1CS::from(r1cs_file);

    let cfg = CircomConfig::<BnScalar> {
        r1cs,
        wtns,
        store,
        sanity_check: false,
    };
    let mut builder = CircomBuilder::new(cfg);

    witness_inputs.iter().for_each(|(name, values)| {
        values.iter().for_each(|value| {
            builder.push_input(name, value.clone());
        });
    });

    let circom = builder.build().unwrap();

    let public_inputs = circom.get_public_inputs().unwrap();

    let mut zkey_file = std::io::Cursor::new(zkey_file);
    let params = ProvingKey::<Bn254>::deserialize_uncompressed_unchecked(&mut zkey_file).unwrap();

    let proof = Groth16::<Bn254, CircomReduction>::prove(&params, circom, &mut rng).unwrap();

    (proof, public_inputs)
}
