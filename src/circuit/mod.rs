use std::marker::PhantomData;

use franklin_crypto::bellman::pairing::Engine;
use franklin_crypto::bellman::{Circuit, ConstraintSystem, SynthesisError};

pub struct SampleCircuit<E: Engine> {
  pub _e: PhantomData<E>,
}

impl<E: Engine> Circuit<E> for SampleCircuit<E> {
  fn synthesize<CS: ConstraintSystem<E>>(self, _cs: &mut CS) -> Result<(), SynthesisError> {
    Ok(())
  }
}
