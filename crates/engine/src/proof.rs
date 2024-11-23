use ark_bn254::{Bn254, Fr as BnScalar, G1Affine, G2Affine};
use ark_groth16::Proof as Groth16Proof;

pub struct Proof {
    pub proof: Groth16Proof<Bn254>,
    pub public_inputs: Vec<BnScalar>,
}

impl Proof {
    pub fn into_json(self) -> serde_json::Value {
        fn g1_to_json(g1: G1Affine) -> Vec<String> {
            vec![g1.x.to_string(), g1.y.to_string()]
        }

        fn g2_to_json(g2: G2Affine) -> Vec<Vec<String>> {
            vec![
                vec![g2.x.c0.to_string(), g2.x.c1.to_string()],
                vec![g2.y.c0.to_string(), g2.y.c1.to_string()],
            ]
        }

        serde_json::json!({
            "proof": {
                "pi_a": g1_to_json(self.proof.a),
                "pi_b": g2_to_json(self.proof.b),
                "pi_c": g1_to_json(self.proof.c),
            },
            "public_inputs": self.public_inputs.iter().map(|input| input.to_string()).collect::<Vec<String>>(),
        })
    }
}