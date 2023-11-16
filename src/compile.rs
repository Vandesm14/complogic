use std::collections::HashMap;

use crate::gates::Gate;
use petgraph::{graph::DiGraph, stable_graph::NodeIndex, Direction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Op {
  /// Performs a Nand operation on two input addresses and stores the result in the output address
  Nand(usize, usize, usize),

  /// Sets the value of the register at the given address
  Set(usize, bool),

  /// Noop
  Noop,
}

impl Default for Op {
  fn default() -> Self {
    Self::Noop
  }
}

pub type Ops = Vec<Op>;

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
pub struct Compiler {
  /// The number of immediate values to allocate when running the simulation
  pub immediate_count: usize,

  /// The ops to run on the registers
  pub ops: Ops,

  /// Incrementer for allocating registers
  pub incrementer: Incrementer,
}

impl Compiler {
  /// Creates a new compiler
  pub fn new(immediate_count: usize) -> Self {
    Self {
      ops: vec![],
      immediate_count,
      incrementer: Incrementer::set(immediate_count),
    }
  }

  /// Resets ops
  pub fn reset_ops(&mut self) {
    self.ops.clear();

    for i in 0..self.immediate_count {
      self.ops.push(Op::Set(i, false));
    }
  }

  /// Resets the incrementer
  pub fn reset_incrementer(&mut self) {
    self.incrementer = Incrementer::set(self.immediate_count);
  }

  /// Allocates a new register and returns its index
  pub fn alloc(&mut self) -> usize {
    self.incrementer.next()
  }

  /// Compiles a list of gates into Ops
  pub fn compile(&mut self, gate: Vec<&Gate>) {
    self.reset_ops();

    // Cloning incrementer since we are generating ops and we don't
    // want to change the incrementer for top-level gates (what we are compiling)
    let mut incrementer = self.incrementer.clone();
    gate.into_iter().for_each(|gate| {
      self.ops.extend(gate.create(&mut incrementer));
    });

    let mut graph =
      DiGraph::<Op, (), usize>::from_edges(self.ops.iter().flat_map(|op| {
        match *op {
          Op::Nand(a, b, out) => {
            vec![(a, out), (b, out)]
          }
          _ => {
            vec![]
          }
        }
      }));
    let mut layers: Vec<Vec<usize>> = vec![vec![]];

    // Add the data for each op
    for op in self.ops.iter() {
      let op = *op;
      match op {
        Op::Nand(_, _, out) => {
          graph[NodeIndex::from(out)] = op;
        }
        Op::Set(reg, _) => {
          graph[NodeIndex::from(reg)] = op;

          layers[0].push(reg);
        }
        Op::Noop => {}
      };
    }

    println!("graph: {graph:#?}");

    for node in layers[0].iter() {
      let children = graph
        .neighbors_directed(NodeIndex::from(*node), Direction::Outgoing)
        .collect::<Vec<_>>();

      println!("Node: {:?}, Neighbors: {:?}", node, children);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{And, RSLatchTest};

  use super::*;

  #[test]
  /// Test the Nand operation and ensure that it works as expected
  fn op_nand() {
    let mut simulation = Simulation {
      registers: vec![false, false, false],
      ops: vec![Op::Nand(0, 1, 2)],
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

    let rslatch = RSLatchTest {
      s,
      r,
      q: simulation.alloc(),
    };

    simulation.compile(vec![&Gate::from(rslatch)]);

    // Reset the latch (due to the nature of logic, it starts as set when it's created)
    simulation.run(&[false, false]);
    assert!(!simulation.registers[rslatch.q]);

    simulation.run(&[true, false]);
    assert!(simulation.registers[rslatch.q]);

    simulation.run(&[false, true]);
    assert!(!simulation.registers[rslatch.q]);

    simulation.run(&[true, true]);
    assert!(!simulation.registers[rslatch.q]);
  }
}
