use std::{
  collections::HashMap,
  rc::Rc,
  sync::atomic::{AtomicUsize, Ordering},
};

use crate::gates::Gate;

#[derive(Debug, Clone)]
pub struct NandOp(pub usize, pub usize, pub usize);
pub type Ops = Vec<NandOp>;

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
    self.val.fetch_add(1, Ordering::Relaxed)
  }

  pub fn next_n(&self, n: usize) -> usize {
    self.val.fetch_add(n, Ordering::Relaxed)
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
}

impl Simulation {
  /// Creates a new simulation
  pub fn new(immediate_count: usize) -> Self {
    Self {
      registers: vec![false; immediate_count],
      ops: vec![],
      immediate_count,
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
  pub fn alloc_one(&mut self) -> usize {
    self.registers.push(false);
    self.registers.len() - 1
  }

  /// Compiles a Gate into Ops (allocation and absolutification)
  pub fn compile_one(&mut self, gate: Rc<dyn Gate>) -> Ops {
    let ops = gate.create(&Incrementer::new());

    // key = relative id, value = absolute id
    let mut rel_to_abs: HashMap<usize, usize> = HashMap::new();
    let ops = ops
      .iter()
      .map(|op| {
        let [a, b, out] = [op.0, op.1, op.2];

        let a = match rel_to_abs.get(&a) {
          Some(id) => *id,
          None => {
            let id = self.alloc_one();
            rel_to_abs.insert(a, id);
            id
          }
        };
        let b = match rel_to_abs.get(&b) {
          Some(id) => *id,
          None => {
            let id = self.alloc_one();
            rel_to_abs.insert(b, id);
            id
          }
        };
        let out = match rel_to_abs.get(&out) {
          Some(id) => *id,
          None => {
            let id = self.alloc_one();
            rel_to_abs.insert(out, id);
            id
          }
        };

        NandOp(a, b, out)
      })
      .collect::<Vec<_>>();

    ops
  }

  /// Compiles a list of gates into Ops
  pub fn compile(&mut self, gate: Vec<Rc<dyn Gate>>) {
    self.registers = vec![false; self.immediate_count];
    self.ops = vec![];

    gate.into_iter().for_each(|gate| {
      let mut ops = self.compile_one(gate);
      self.ops.append(&mut ops);
    });
  }

  pub fn add_op(&mut self, op: NandOp) {
    self.ops.push(op);
  }

  pub fn register(&self, index: usize) -> bool {
    self.registers[index]
  }
}
