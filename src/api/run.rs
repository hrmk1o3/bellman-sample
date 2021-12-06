use std::marker::PhantomData;
use std::path::Path;

use anyhow::Result;
use franklin_crypto::bellman::groth16::{
  create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::pairing::ff::{Field, PrimeField};
use franklin_crypto::bellman::pairing::Engine;
use franklin_crypto::circuit::multipack::bytes_to_bits_le;

use crate::circuit::SampleCircuit;

use super::input::CircuitInput;

pub fn run(circuit_input: CircuitInput) -> Result<()> {
  // setup
  let dummy_input = CircuitInput::default();
  let circuit = SampleCircuit {
    inputs: dummy_input.inputs,
    _e: PhantomData::<Bn256>,
  };
  let parameters = generate_random_parameters::<Bn256, _, _>(circuit, &mut rand::thread_rng())?;

  // prove
  let circuit = SampleCircuit {
    inputs: circuit_input.inputs,
    _e: PhantomData::<Bn256>,
  };
  let public_wires_bytes = circuit.get_public_wires()?;
  let public_inputs = decode_public_wires::<Bn256>(&public_wires_bytes);
  let proof = create_random_proof(circuit, &parameters, &mut rand::thread_rng())?;

  // verify
  let vk = parameters.vk;
  let verifying_key = prepare_verifying_key(&vk);
  let success = verify_proof(&verifying_key, &proof, &public_inputs).unwrap();
  assert!(success, "verification error");

  Ok(())
}

pub fn run_with_file(input_path: &Path) -> Result<()> {
  let circuit_input = CircuitInput::from_path(input_path)?;
  run(circuit_input)
}

pub fn decode_public_wires<E: Engine>(bytes: &[u8]) -> Vec<E::Fr> {
  let capacity_bytes = (E::Fr::CAPACITY - 1) / 8 + 1;
  let mut result: Vec<E::Fr> = vec![];

  for split_bytes in bytes.chunks(capacity_bytes as usize) {
    let mut cur = E::Fr::zero();
    let mut coeff = E::Fr::one();

    for bit in bytes_to_bits_le(split_bytes) {
      if bit {
        cur.add_assign(&coeff);
      }

      coeff.double();
    }

    result.push(cur);
  }

  result
}
