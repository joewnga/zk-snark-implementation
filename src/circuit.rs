use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable};
use ark_r1cs_std::prelude::*;
use ark_ff::Field;


pub struct ArithmeticCircuit<F: Field> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub c: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for ArithmeticCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_input_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_input_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| self.c.ok_or(SynthesisError::AssignmentMissing))?;

        
        cs.enforce_constraint(
            lc!() + a + b,
            lc!() + Variable::One,
            lc!() + c,
        )?;

        Ok(())
    }
}
