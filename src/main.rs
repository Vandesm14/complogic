use std::fmt::Debug;

#[derive(Debug, Clone)]
enum Op {
  Set(usize, bool),
  Call(Call),
}

// TODO: merge these into the `Op` enum
#[derive(Debug, Clone)]
enum Call {
  And(usize, usize, usize),
  Not(usize, usize),
}

type Ops = Vec<Op>;

enum Gate {
  And(usize, usize),
  Not(usize),

  Static(bool),
}

#[derive(Debug)]
struct Simulation {
  /// Registers that note the inputs and outputs of logic gates
  registers: Vec<bool>,

  /// The ops to run on the registers
  ops: Ops,
}

impl Simulation {
  /// Runs the VM with the given ops
  fn run(&mut self) {
    self.ops.iter().for_each(|op| match *op {
      Op::Call(ref call) => match *call {
        Call::And(a, b, out) => {
          let a = self.registers[a];
          let b = self.registers[b];
          self.registers[out] = a && b;
        }
        Call::Not(a, out) => {
          let a = self.registers[a];
          self.registers[out] = !a;
        }
      },
      Op::Set(register, value) => self.registers[register] = value,
    });
  }

  fn add_gate(&mut self, gate: Gate) -> usize {
    match gate {
      Gate::And(a, b) => {
        // Allocate an output register
        self.registers.push(false);
        let out = self.registers.len() - 1;

        // Add the op
        self.ops.push(Op::Call(Call::And(a, b, out)));

        // Return the output register
        out
      }
      Gate::Not(a) => {
        // Allocate an output register
        self.registers.push(false);
        let out = self.registers.len() - 1;

        // Add the op
        self.ops.push(Op::Call(Call::Not(a, out)));

        // Return the output register
        out
      }
      Gate::Static(value) => {
        // Allocate an output register
        self.registers.push(value);
        let out = self.registers.len() - 1;

        // Add the op
        self.ops.push(Op::Set(out, value));

        // Return the output register
        out
      }
    }
  }
}

fn main() {
  let mut simulation = Simulation {
    registers: vec![],
    ops: vec![],
  };

  let a = simulation.add_gate(Gate::Static(true));
  let b = simulation.add_gate(Gate::Static(false));

  let out = simulation.add_gate(Gate::And(a, b));
  simulation.add_gate(Gate::Not(out));

  simulation.run();
  println!("Simulation: {:?}", simulation);
  println!("Static, Static, And, Not");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn op_and() {
    let mut simulation = Simulation {
      registers: vec![false; 3],
      ops: vec![
        Op::Set(0, true),
        Op::Set(1, true),
        Op::Call(Call::And(0, 1, 2)),
      ],
    };

    simulation.run();
    assert!(simulation.registers[2]);
  }

  #[test]
  fn op_and_false() {
    let mut simulation = Simulation {
      registers: vec![false; 3],
      ops: vec![
        Op::Set(0, true),
        Op::Set(1, false),
        Op::Call(Call::And(0, 1, 2)),
      ],
    };

    simulation.run();
    assert!(!simulation.registers[2]);
  }

  #[test]
  fn op_not() {
    let mut simulation = Simulation {
      registers: vec![false; 2],
      ops: vec![Op::Set(0, true), Op::Call(Call::Not(0, 1))],
    };

    simulation.run();
    assert!(!simulation.registers[1]);
  }

  #[test]
  fn op_not_false() {
    let mut simulation = Simulation {
      registers: vec![false; 2],
      ops: vec![Op::Set(0, false), Op::Call(Call::Not(0, 1))],
    };

    simulation.run();
    assert!(simulation.registers[1]);
  }
}
