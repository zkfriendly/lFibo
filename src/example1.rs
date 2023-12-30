mod fibo_circuit;

use halo2_proofs::{dev::MockProver, pasta::Fp};

use crate::fibo_circuit::FiboCircuit;

fn main() {
    println!("Setting up the circuit...");
    let k = 4;
    let a = Fp::from(1);
    let b = Fp::from(1);
    let out = Fp::from(55);

    let circuit = FiboCircuit {
        a: Some(a),
        b: Some(b),
    };

    let public_input = vec![a, b, out];

    println!("Calculating the proof...");
    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();

    println!("Verifing proof...");
    prover.assert_satisfied();

    println!("Proof is verified!");
}
