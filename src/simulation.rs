use crate::gates::Gate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct NandOp(pub usize, pub usize, pub usize);
pub type Ops = Vec<NandOp>;

fn move_element<T>(vec: &mut Vec<T>, from_index: usize, to_index: usize) {
  // TODO: fix the clippy thing
  #[allow(clippy::comparison_chain)]
  if from_index < to_index {
    let element = vec.remove(from_index);
    vec.insert(to_index - 1, element);
  } else if from_index > to_index {
    let element = vec.remove(from_index);
    vec.insert(to_index, element);
  }
}

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

    #[cfg(test)]
    println!("Start: {:?}", self.ops);

    for op in self.ops.clone().into_iter() {
      let a = op.0;
      let b = op.1;
      let our_index = self.ops.iter().position(|o| *o == op).unwrap();

      if a < self.immediate_count && b < self.immediate_count {
        move_element(&mut self.ops, our_index, 0);
        continue;
      }

      let a_index = self.ops.iter().position(|op| op.2 == a).unwrap_or(0);
      let b_index = self.ops.iter().position(|op| op.2 == b).unwrap_or(0);

      let max = a_index.max(b_index);

      if our_index < max {
        move_element(&mut self.ops, our_index, max + 1);
      }
    }

    #[cfg(test)]
    println!("End: {:?}", self.ops);

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
  use crate::{And, Nor, Or};

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
  /// Custom compiling a RS Latch that has self-referencing gates
  fn non_sorted_gates_should_sort() {
    let mut simulation = Simulation::new(2);
    let [s, r] = [0, 1];

    let q_patch = simulation.alloc();
    let qn_patch = simulation.alloc();

    let nor_1 = Nor {
      a: s,
      b: qn_patch,
      out: q_patch,
    };
    let nor_2 = Nor {
      a: r,
      b: q_patch,
      out: qn_patch,
    };

    let or_q = Or {
      a: qn_patch,
      b: qn_patch,
      out: simulation.alloc(),
    };

    simulation
      .ops
      .extend(Gate::from(or_q).create(&mut simulation.incrementer));
    simulation
      .ops
      .extend(Gate::from(nor_2).create(&mut simulation.incrementer));
    simulation
      .ops
      .extend(Gate::from(nor_1).create(&mut simulation.incrementer));
    simulation.registers = vec![false; 13];

    // Reset the latch (due to the nature of logic, it starts as set when it's created)
    simulation.run(&[false, false]);
    assert!(!simulation.registers[or_q.out]);

    simulation.run(&[true, false]);
    assert!(simulation.registers[or_q.out]);

    simulation.run(&[false, true]);
    assert!(!simulation.registers[or_q.out]);

    simulation.run(&[true, true]);
    assert!(!simulation.registers[or_q.out]);
  }
}
