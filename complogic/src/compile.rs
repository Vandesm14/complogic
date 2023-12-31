use std::{collections::HashSet, fs};

use crate::{gates::Gate, Simulation};
use petgraph::{dot::Dot, graph::DiGraph, stable_graph::NodeIndex, Direction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Op {
  /// Performs a Nand operation on two input addresses and stores the result in the output address
  Nand(usize, usize, usize),

  /// Sets the value of the register at the given address
  Set(usize, bool),
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

    let mut graph = DiGraph::<Op, (), usize>::default();
    self.ops.iter().for_each(|op| {
      graph.add_node(*op);
    });

    self.ops.iter().for_each(|op| {
      if let Op::Nand(a, b, out) = op {
        if *a < graph.node_count()
          && *b < graph.node_count()
          && *out < graph.node_count()
        {
          graph.add_edge(NodeIndex::from(*a), NodeIndex::from(*out), ());
          graph.add_edge(NodeIndex::from(*b), NodeIndex::from(*out), ());
        }
      }
    });

    let mut ops: Vec<Op> = vec![];
    let mut nodes_to_process: HashSet<usize> = HashSet::default();
    let mut queue: Vec<usize> = vec![];
    let mut next_queue: Vec<usize> = vec![];

    // Add the data for each op
    for op in self.ops.iter() {
      let op = *op;
      match op {
        Op::Nand(_, _, out) => {
          graph[NodeIndex::from(out)] = op;
          nodes_to_process.insert(out);
        }
        Op::Set(reg, _) => {
          graph[NodeIndex::from(reg)] = op;

          // Add the output of the immediate to the layers
          queue.push(reg);
          nodes_to_process.insert(reg);
        }
      };
    }

    fs::write("graph.dot", format!("{:?}", Dot::with_config(&graph, &[])))
      .expect("Unable to write file");

    // Flag to force-add all gates in the queue if recursion is detected
    let mut recursion_flag = false;
    loop {
      for node in queue.iter() {
        let node = *node;
        if !nodes_to_process.contains(&node) {
          continue;
        }

        let mut inputs =
          graph.neighbors_directed(NodeIndex::from(node), Direction::Incoming);

        let requires_new_layer = if recursion_flag {
          false
        } else {
          inputs.any(|i| nodes_to_process.contains(&i.index()))
        };

        if requires_new_layer {
          next_queue.push(node);
        } else {
          ops.push(graph[NodeIndex::from(node)]);
          nodes_to_process.remove(&node);

          graph
            .neighbors_directed(NodeIndex::from(node), Direction::Outgoing)
            .for_each(|n| {
              next_queue.push(n.index());
            });
        }
      }

      queue.sort();
      next_queue.sort();

      recursion_flag = queue == next_queue;

      std::mem::swap(&mut queue, &mut next_queue);
      next_queue.clear();

      if queue.is_empty() {
        break;
      }
    }

    Simulation {
      registers: vec![false; incrementer.val],
      ops,
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
