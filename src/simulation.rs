use crate::gates::Gate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NandOp(pub usize, pub usize, pub usize);
pub type Ops = Vec<NandOp>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incrementer {
  pub val: usize,
}

impl Incrementer {
  pub fn new() -> Self {
    Self { val: 0 }
  }

  pub fn set(val: usize) -> Self {
    Self { val }
  }

  #[allow(clippy::should_implement_trait)]
  pub fn next(&mut self) -> usize {
    let current = self.val;
    self.val += 1;

    current
  }

  pub fn skip(&mut self, count: usize) {
    self.val += count;
  }
}

impl Default for Incrementer {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Simulation {
  /// Registers that note the inputs and outputs of logic gates
  pub registers: Vec<bool>,

  /// The number of immediate values to allocate when running the simulation
  pub immediate_count: usize,

  /// The ops to run on the registers
  pub ops: Ops,

  /// Incrementer for allocating registers
  pub incrementer: Incrementer,
}

impl Simulation {
  /// Creates a new simulation
  pub fn new(immediate_count: usize) -> Self {
    Self {
      registers: vec![],
      ops: vec![],
      immediate_count,
      incrementer: Incrementer::set(immediate_count),
    }
  }

  /// Resets the incrementer
  pub fn reset_incrementer(&mut self) {
    self.incrementer = Incrementer::set(self.immediate_count);
  }

  /// Runs the VM with the given immediates
  pub fn run(&mut self, immediates: &[bool]) {
    if immediates.len() != self.immediate_count {
      panic!(
        "Expected {} immediates, got {}",
        self.immediate_count,
        immediates.len()
      );
    }

    immediates.iter().enumerate().for_each(|(i, value)| {
      self.registers[i] = *value;
    });

    self.ops.iter().for_each(|op| match *op {
      NandOp(a, b, out) => {
        let a = self.registers[a];
        let b = self.registers[b];
        self.registers[out] = !(a && b);
      }
    });
  }

  /// Allocates a new register and returns its index
  pub fn alloc(&mut self) -> usize {
    self.incrementer.next()
  }

  /// Compiles a list of gates into Ops
  pub fn compile(&mut self, gate: Vec<&Gate>) -> usize {
    self.ops = vec![];

    // Cloning incrementer since we are generating ops and we don't
    // want to change the incrementer for top-level gates (what we are compiling)
    let mut incrementer = self.incrementer.clone();
    gate.into_iter().for_each(|gate| {
      self.ops.extend(gate.create(&mut incrementer));
    });

    let reg_count = incrementer.val;
    self.registers = vec![false; reg_count];

    reg_count
  }

  pub fn add_op(&mut self, op: NandOp) {
    self.ops.push(op);
  }

  pub fn register(&self, index: usize) -> bool {
    self.registers[index]
  }
}

#[cfg(test)]
mod tests {
  use crate::And;

  use super::*;

  #[test]
  /// Test the Nand operation and ensure that it works as expected
  fn op_nand() {
    let mut simulation = Simulation {
      registers: vec![false, false, false],
      ops: vec![NandOp(0, 1, 2)],
      immediate_count: 2,
      incrementer: Incrementer::set(2 - 1),
    };

    simulation.run(&[false, false]);
    assert!(simulation.registers[2]);

    simulation.run(&[true, false]);
    assert!(simulation.registers[2]);

    simulation.run(&[false, true]);
    assert!(simulation.registers[2]);

    simulation.run(&[true, true]);
    assert!(!simulation.registers[2]);
  }

  #[test]
  /// Test that allocation increments properly and doesn't allocate any registers
  fn alloc_lazy_increment() {
    let mut simulation = Simulation::new(0);
    assert_eq!(simulation.alloc(), 0);
    assert_eq!(simulation.alloc(), 1);
    assert_eq!(simulation.alloc(), 2);

    // Ensure that no registers are actually created
    assert_eq!(simulation.registers.len(), 0);
  }

  #[test]
  /// Test that allocation increments plus the immediate count
  fn alloc_plus_immediates() {
    let mut simulation = Simulation::new(2);
    assert_eq!(simulation.alloc(), 2);
    assert_eq!(simulation.alloc(), 3);
    assert_eq!(simulation.alloc(), 4);

    // Ensure that no registers are actually created
    assert_eq!(simulation.registers.len(), 0);
  }

  #[test]
  /// Test that the registers we allocate at compile time are the same as the registers we allocate
  fn alloc_all_on_compile() {
    let mut simulation = Simulation::new(2);

    // Two immediates = two registers allocated
    let reg_count = simulation.compile(vec![]);
    assert_eq!(reg_count, 2);

    // Two immediates = the next index should be 2, then 3
    assert_eq!(simulation.alloc(), 2);
    assert_eq!(simulation.alloc(), 3);

    // Two immediates plus our allocs, total register count is 4
    let reg_count = simulation.compile(vec![]);
    assert_eq!(reg_count, 4);
  }

  #[test]
  /// Test that compiling doesn't increment the incrementer
  fn keep_incrementer_on_compile() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let and = And {
      a,
      b,
      out: simulation.alloc(),
    };

    // When compiling, we should not increment the incrementer
    simulation.compile(vec![&Gate::from(and)]);
    simulation.compile(vec![&Gate::from(and)]);
    let reg_count = simulation.compile(vec![&Gate::from(and)]);

    // Two immediates plus two outputs for the And's internal Nand gates
    assert_eq!(reg_count, 4);
  }

  #[test]
  /// Test that mis-ordering gates doesn't break the simulation
  fn mis_ordered_gates() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let and = And {
      a,
      b,
      out: simulation.alloc(),
    };

    let and_2 = And {
      a: and.out,
      b,
      out: simulation.alloc(),
    };

    // Order the second And before the first, even though
    // it should be the other way around.
    simulation.compile(vec![&Gate::from(and_2), &Gate::from(and)]);

    // Check the false state for sanity (both should be false)
    simulation.run(&[false, false]);
    assert!(!simulation.registers[and.out]);
    assert!(!simulation.registers[and_2.out]);

    // The compiler should order them correctly
    // so both gates should be true
    simulation.run(&[true, true]);
    assert!(simulation.registers[and.out]);
    assert!(simulation.registers[and_2.out]);
  }
}
