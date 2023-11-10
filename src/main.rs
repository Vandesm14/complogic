use std::fmt::Debug;

#[derive(Debug, Clone)]
enum Op {
  And(usize, usize, usize),
  Not(usize, usize),
}

type Ops = Vec<Op>;

enum Gate {
  And(usize, usize),
  Not(usize),
  Or(usize, usize),
}

impl Gate {
  fn add_to(&self, out: usize, simulation: &mut Simulation) {
    match *self {
      Self::And(a, b) => simulation.add_op(Op::And(a, b, out)),
      Self::Not(a) => simulation.add_op(Op::Not(a, out)),
      Self::Or(a, b) => {
        let not_a = simulation.add_gate(Gate::Not(a));
        let not_b = simulation.add_gate(Gate::Not(b));
        let and = simulation.add_gate(Gate::And(not_a, not_b));

        simulation.add_gate_with_out(Gate::Not(and), out);
      }
    }
  }
}

#[derive(Debug)]
struct Simulation {
  /// Registers that note the inputs and outputs of logic gates
  registers: Vec<bool>,

  /// The ops to run on the registers
  ops: Ops,

  /// The number of immediate values to allocate when running the simulation
  immediate_count: usize,
}

impl Simulation {
  /// Creates a new simulation
  fn new(immediate_count: usize) -> Self {
    Self {
      registers: vec![false; immediate_count],
      ops: vec![],
      immediate_count,
    }
  }

  /// Runs the VM with the given immediates
  fn run(&mut self, immediates: &[bool]) {
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
      Op::And(a, b, out) => {
        let a = self.registers[a];
        let b = self.registers[b];
        self.registers[out] = a && b;
      }
      Op::Not(a, out) => {
        let a = self.registers[a];
        self.registers[out] = !a;
      }
    });
  }

  /// Allocates a new register and returns its index
  fn alloc_one(&mut self) -> usize {
    self.registers.push(false);
    self.registers.len() - 1
  }

  /// Adds a gate to the simulation and returns the output register
  fn add_gate(&mut self, gate: Gate) -> usize {
    let out = self.alloc_one();
    gate.add_to(out, self);

    out
  }

  /// Adds a gate and uses an existing register as the output
  fn add_gate_with_out(&mut self, gate: Gate, out: usize) {
    gate.add_to(out, self);
  }

  fn add_op(&mut self, op: Op) {
    self.ops.push(op);
  }
}

fn main() {
  let mut simulation = Simulation::new(2);
  let [a, b] = [0, 1];

  let out = simulation.add_gate(Gate::And(a, b));
  simulation.add_gate(Gate::Not(out));

  simulation.run(&[true, false]);
  println!("Simulation: {:?}", simulation);
  println!("Static, Static, And, Not");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn op_and() {
    let mut simulation = Simulation {
      registers: vec![true, true, false],
      ops: vec![Op::And(0, 1, 2)],
      immediate_count: 0,
    };

    simulation.run(&[]);
    assert!(simulation.registers[2]);
  }

  #[test]
  fn op_and_false() {
    let mut simulation = Simulation {
      registers: vec![true, false, false],
      ops: vec![Op::And(0, 1, 2)],
      immediate_count: 0,
    };

    simulation.run(&[]);
    assert!(!simulation.registers[2]);
  }

  #[test]
  fn op_not() {
    let mut simulation = Simulation {
      registers: vec![true, false],
      ops: vec![Op::Not(0, 1)],
      immediate_count: 0,
    };

    simulation.run(&[]);
    assert!(!simulation.registers[1]);
  }

  #[test]
  fn op_not_false() {
    let mut simulation = Simulation {
      registers: vec![false, false],
      ops: vec![Op::Not(0, 1)],
      immediate_count: 0,
    };

    simulation.run(&[]);
    assert!(simulation.registers[1]);
  }

  #[test]
  fn add_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let out = simulation.add_gate(Gate::And(a, b));

    simulation.run(&[true, true]);
    assert!(simulation.registers[out]);

    simulation.run(&[true, false]);
    assert!(!simulation.registers[out]);
  }

  #[test]
  fn add_gate_with_out() {
    let mut simulation = Simulation::new(0);

    let a = simulation.alloc_one();

    simulation.add_gate_with_out(Gate::Not(a), a);

    simulation.run(&[]);
    assert!(simulation.registers[a]);

    simulation.run(&[]);
    assert!(!simulation.registers[a]);

    simulation.run(&[]);
    assert!(simulation.registers[a]);
  }

  #[test]
  fn or_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let out = simulation.add_gate(Gate::Or(a, b));

    simulation.run(&[true, false]);
    assert!(simulation.registers[out]);
  }

  #[test]
  fn or_gate_false() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let out = simulation.add_gate(Gate::Or(a, b));

    simulation.run(&[false, false]);
    assert!(!simulation.registers[out]);
  }
}
