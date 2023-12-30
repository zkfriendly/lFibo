mod fibo_circuit;

use halo2_proofs::{dev::MockProver, pasta::Fp};

use crate::fibo_circuit::FiboCircuit;

fn main() {
    println!("Setting up the circuit...");
    let k = 4;
    let a = Fp::from(1);
    let b = Fp::from(1);

    let circuit = FiboCircuit {
        a: Some(a),
        b: Some(b),
    };

    println!("Calculating the proof...");
    let prover = MockProver::run(k, &circuit, vec![]).unwrap();

    prover.assert_satisfied();
    println!("The proof is valid!");
}
