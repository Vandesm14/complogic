use std::{
  rc::Rc,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::gates::Gate;

#[derive(Debug, Clone)]
pub struct NandOp(pub usize, pub usize, pub usize);
pub type Ops = Vec<NandOp>;

#[derive(Debug)]
pub struct Incrementer {
  pub val: AtomicUsize,
}

impl Incrementer {
  pub fn new() -> Self {
    Self {
      val: AtomicUsize::new(0),
    }
  }

  pub fn set(val: usize) -> Self {
    Self {
      val: AtomicUsize::new(val),
    }
  }

  pub fn next(&self) -> usize {
    self.val.fetch_add(1, Ordering::AcqRel)
  }

  pub fn next_n(&self, n: usize) -> usize {
    self.val.fetch_add(n, Ordering::AcqRel) + n - 1
  }
}

impl Default for Incrementer {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug)]
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
      registers: vec![false; immediate_count],
      ops: vec![],
      immediate_count,
      incrementer: Incrementer::set(immediate_count),
    }
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
  pub fn compile(&mut self, gate: Vec<Rc<dyn Gate>>) -> usize {
    self.ops = vec![];

    gate.into_iter().for_each(|gate| {
      self.ops.extend(gate.create(&self.incrementer));
    });

    let reg_count = self.incrementer.val.load(Ordering::Relaxed);
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
  use super::*;

  #[test]
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
  fn alloc_lazy_increment() {
    let mut simulation = Simulation::new(0);
    assert_eq!(simulation.registers.len(), 0);
    assert_eq!(simulation.alloc(), 0);
    assert_eq!(simulation.alloc(), 1);
    assert_eq!(simulation.alloc(), 2);
  }

  #[test]
  fn alloc_plus_immediates() {
    let mut simulation = Simulation::new(2);
    assert_eq!(simulation.alloc(), 2);
    assert_eq!(simulation.alloc(), 3);
    assert_eq!(simulation.alloc(), 4);
  }

  #[test]
  fn alloc_all_on_compile() {
    let mut simulation = Simulation::new(2);
    assert_eq!(simulation.registers.len(), 2);

    let reg_count = simulation.compile(vec![]);
    assert_eq!(reg_count, 2);

    assert_eq!(simulation.alloc(), 2);
    assert_eq!(simulation.alloc(), 3);

    let reg_count = simulation.compile(vec![]);
    assert_eq!(reg_count, 4);
  }
}
