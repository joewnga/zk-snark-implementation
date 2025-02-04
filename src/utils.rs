use ark_ff::{Field, UniformRand};
use ark_std::rand::thread_rng;

pub fn random_field_element<F: Field>() -> F {
    let mut rng = thread_rng();
    F::rand(&mut rng)
}
