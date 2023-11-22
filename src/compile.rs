use std::fs;

use crate::{gates::Gate, Simulation};
use eframe::epaint::ahash::HashSet;
use petgraph::{dot::Dot, graph::DiGraph, stable_graph::NodeIndex, Direction};
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

#[derive(Debug, Default)]
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
  pub fn compile(&mut self, gates: Vec<&Gate>) -> Simulation {
    self.reset_ops();

    if gates.is_empty() {
      return Simulation {
        registers: vec![false; self.immediate_count],
        ops: vec![],
      };
    }

    // Cloning incrementer since we are generating ops and we don't
    // want to change the incrementer for top-level gates (what we are compiling)
    let mut incrementer = self.incrementer.clone();
    gates.into_iter().for_each(|gate| {
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

    let mut layers: Vec<HashSet<usize>> = vec![];

    let mut all_seen_nodes: HashSet<usize> = HashSet::default();
    let mut current_layer = 0;
    let mut queue: Vec<usize> = vec![];
    let mut next_queue: Vec<usize> = vec![];

    // Add the data for each op
    for op in self.ops.iter() {
      let op = *op;
      match op {
        Op::Nand(_, _, out) => {
          graph[NodeIndex::from(out)] = op;
        }
        Op::Set(reg, _) => {
          graph[NodeIndex::from(reg)] = op;

          // Add the output of the immediate to the layers
          queue.push(reg);
        }
        Op::Noop => {}
      };
    }

    fs::write("graph.dot", format!("{:?}", Dot::with_config(&graph, &[])))
      .expect("Unable to write file");

    loop {
      layers.push(HashSet::default());
      while let Some(node) = queue.pop() {
        if all_seen_nodes.contains(&node) {
          continue;
        }

        let children = graph
          .neighbors_directed(NodeIndex::from(node), Direction::Outgoing)
          .collect::<Vec<_>>();

        children.iter().for_each(|n| {
          next_queue.push(n.index());
        });

        let inputs = graph
          .neighbors_directed(NodeIndex::from(node), Direction::Incoming)
          .collect::<Vec<_>>();

        let requires_new_layer =
          queue.iter().any(|n| inputs.iter().any(|i| i.index() == *n));
        if requires_new_layer {
          if layers.len() == current_layer + 1 {
            layers.push(HashSet::default());
          }

          next_queue.push(node);
        } else {
          layers[current_layer].insert(node);
          all_seen_nodes.insert(node);
        }
      }

      queue.clear();
      queue.extend(next_queue.iter());

      next_queue.clear();

      current_layer += 1;

      if queue.is_empty() {
        break;
      }
    }

    Simulation {
      registers: vec![false; incrementer.val],
      ops: layers
        .into_iter()
        .flat_map(|layer| layer.into_iter().map(|n| graph[NodeIndex::from(n)]))
        .collect(),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{And, RSLatchTest};

  use super::*;

  #[test]
  /// Test that allocation increments properly and doesn't allocate any registers
  fn alloc_lazy_increment() {
    let mut compiler = Compiler::new(0);
    assert_eq!(compiler.alloc(), 0);
    assert_eq!(compiler.alloc(), 1);
    assert_eq!(compiler.alloc(), 2);
  }

  #[test]
  /// Test that allocation increments plus the immediate count
  fn alloc_plus_immediates() {
    let mut compiler = Compiler::new(2);
    assert_eq!(compiler.alloc(), 2);
    assert_eq!(compiler.alloc(), 3);
    assert_eq!(compiler.alloc(), 4);
  }

  #[test]
  /// Test that the registers we allocate at compile time are the same as the registers we allocate
  fn alloc_all_on_compile() {
    let mut compiler = Compiler::new(2);

    // Two immediates = two registers allocated
    let simulation = compiler.compile(vec![]);
    assert_eq!(simulation.registers.len(), 2);

    // Two immediates = the next index should be 2, then 3
    assert_eq!(compiler.alloc(), 2);
    assert_eq!(compiler.alloc(), 3);

    // Two immediates and it ignores the registers we allocated
    let simulation = compiler.compile(vec![]);
    assert_eq!(simulation.registers.len(), 2);
  }

  #[test]
  /// Test that compiling doesn't increment the incrementer
  fn keep_incrementer_on_compile() {
    let mut compiler = Compiler::new(2);
    let [a, b] = [0, 1];

    let and = And {
      a,
      b,
      out: compiler.alloc(),
    };

    // When compiling, we should not increment the incrementer
    compiler.compile(vec![&Gate::from(and)]);
    compiler.compile(vec![&Gate::from(and)]);
    let simulation = compiler.compile(vec![&Gate::from(and)]);

    // Two immediates plus two outputs for the And's internal Nand gates
    assert_eq!(simulation.registers.len(), 4);
  }

  #[test]
  /// Custom compiling a RS Latch that has self-referencing gates
  fn non_sorted_gates_should_sort() {
    let mut compiler = Compiler::new(2);
    let [s, r] = [0, 1];

    let rslatch = RSLatchTest {
      s,
      r,
      q: compiler.alloc(),
    };

    let mut simulation = compiler.compile(vec![&Gate::from(rslatch)]);

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
