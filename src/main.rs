use std::path::Path;

use bellman_sample::api::{
  prove::create_random_proof_with_file, setup::generate_random_parameters_with_file,
  verify::verify_proof_with_file,
};

fn main() {
  let input_path = Path::new("./tests/input.json");
  let pk_path = Path::new("./tests/proving_key");
  let vk_path = Path::new("./tests/verifying_key");
  let proof_path = Path::new("./tests/proof");
  let public_wires_path = Path::new("./tests/public_wires.txt");
  generate_random_parameters_with_file(pk_path, vk_path).unwrap();
  create_random_proof_with_file(pk_path, input_path, proof_path, public_wires_path).unwrap();
  verify_proof_with_file(vk_path, proof_path, public_wires_path).unwrap();
}
