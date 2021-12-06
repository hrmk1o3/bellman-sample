use std::marker::PhantomData;

use franklin_crypto::bellman::pairing::Engine;
use franklin_crypto::bellman::{Circuit, ConstraintSystem, SynthesisError};

// the circuit to prove a prover knows `inputs` that `output` is equal to the logical AND of
// `inputs[0]` and `inputs[1]`
pub struct SampleCircuit<E: Engine> {
  pub _e: PhantomData<E>,
}

impl<E: Engine> Circuit<E> for SampleCircuit<E> {
  fn synthesize<CS: ConstraintSystem<E>>(self, _cs: &mut CS) -> Result<(), SynthesisError> {
    // `inputs[0]` and `inputs[1]` are boolean values.
    // `output` is the logical AND of `inputs[0]` and `inputs[1]`.
    // `output` is a public variable, while `inputs[0]` and `inputs[1]` are private variables.

    Ok(())
  }
}
