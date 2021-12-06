use bellman_sample::api::{input::CircuitInput, run::run};

fn main() {
  let circuit_input = CircuitInput {
    inputs: [Some(true), Some(true)],
  };
  run(circuit_input).unwrap();
}
