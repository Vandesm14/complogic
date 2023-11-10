use std::fmt::Debug;

#[derive(Debug, Clone)]
enum Op {
  Set(usize, bool),
  Call(Call),
}

#[derive(Debug, Clone)]
enum Call {
  And(usize, usize, usize),
  Not(usize, usize),
}

type Ops = Vec<Op>;

#[derive(Debug)]
struct Simulation {
  /// Registers
  registers: Vec<bool>,

  /// A list of the ops
  ops: Ops,
}

impl Simulation {
  // TODO: give better name
  /// Arbitrary run for right now
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
}

fn main() {
  let mut simulation = Simulation {
    registers: vec![false; 4],
    ops: vec![
      Op::Set(0, true),
      Op::Set(1, true),
      Op::Call(Call::And(0, 1, 2)),
      Op::Call(Call::Not(2, 3)),
    ],
  };

  simulation.run();
  println!("Simulation: {:?}", simulation);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn and_gate() {
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
  fn and_gate_false() {
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
  fn not_gate() {
    let mut simulation = Simulation {
      registers: vec![false; 2],
      ops: vec![Op::Set(0, true), Op::Call(Call::Not(0, 1))],
    };

    simulation.run();
    assert!(!simulation.registers[1]);
  }

  #[test]
  fn not_gate_false() {
    let mut simulation = Simulation {
      registers: vec![false; 2],
      ops: vec![Op::Set(0, false), Op::Call(Call::Not(0, 1))],
    };

    simulation.run();
    assert!(simulation.registers[1]);
  }
}
