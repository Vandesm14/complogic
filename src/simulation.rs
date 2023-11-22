use crate::Op;

#[derive(Debug, Clone)]
pub struct Simulation {
  /// Stores the IDs of the nodes to evaluate in parallel
  /// It is ordered in last to first, where the first is popped off
  pub layers: Vec<Vec<Op>>,

  /// Stores the values of the registers
  pub registers: Vec<bool>,
}

impl Simulation {
  fn run(immediates: &[usize]) {
    todo!()
  }
}
