pub struct CircuitInput {
  pub inputs: [Option<bool>; 2],
}

impl CircuitInput {
  pub fn default() -> Self {
    Self {
      inputs: [None, None],
    }
  }
}
