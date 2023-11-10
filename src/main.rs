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
