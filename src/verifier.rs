use ark_groth16::{VerifyingKey, Proof, Groth16};
use ark_relations::r1cs::ConstraintSystemRef;
use ark_ff::Field;

pub fn verify_proof<F: Field>(
    vk: &VerifyingKey<F>,
    proof: &Proof<F>,
    public_inputs: &[F],
) -> bool {
    let cs = ConstraintSystemRef::new();
    Groth16::verify(vk, proof, &cs, public_inputs).unwrap_or(false)
}
