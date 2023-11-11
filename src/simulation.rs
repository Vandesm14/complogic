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
    self.val.fetch_add(n, Ordering::AcqRel)
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
      // Subtracting 1 since count is 1-indexed
      incrementer: Incrementer::set(immediate_count - 1),
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
    self.registers.push(false);
    self.incrementer.next();
    self.registers.len() - 1
  }

  /// Compiles a list of gates into Ops
  pub fn compile(&mut self, gate: Vec<Rc<dyn Gate>>) {
    self.ops = vec![];
    let mut max_index = self.immediate_count;

    gate.into_iter().for_each(|gate| {
      let ops = gate.create(&self.incrementer);

      ops.iter().for_each(|op| {
        max_index = op.0.max(op.1).max(op.2).max(max_index);
      });

      self.ops.extend(ops);
    });

    self.registers = vec![false; max_index + 1];
  }

  pub fn add_op(&mut self, op: NandOp) {
    self.ops.push(op);
  }

  pub fn register(&self, index: usize) -> bool {
    self.registers[index]
  }
}
