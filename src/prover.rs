use ark_groth16::{ProvingKey, Proof, Groth16};
use ark_std::rand::thread_rng;
use ark_ff::UniformRand;
use ark_relations::r1cs::ConstraintSystem;
use crate::circuit::ArithmeticCircuit;

pub fn generate_proof<F: ark_ff::Field>(
    pk: &ProvingKey<F>,
    a: F, b: F, c: F
) -> Proof<F> {
    let cs = ConstraintSystem::new_ref();
    let circuit = ArithmeticCircuit { a: Some(a), b: Some(b), c: Some(c) };

    circuit.generate_constraints(cs.clone()).unwrap();
    let mut rng = thread_rng();

    Groth16::prove(pk, cs, &mut rng).expect("Proof generation failed")
}
