use crate::Op;

#[derive(Debug, Clone)]
pub struct Simulation {
  /// Stores the ops to evaluate
  pub ops: Vec<Op>,

  /// Stores the values of the registers
  pub registers: Vec<bool>,
}

impl Simulation {
  pub fn run(&mut self, immediates: &[bool]) {
    for op in self.ops.iter() {
      match *op {
        Op::Nand(a, b, out) => {
          let a = self.registers[a];
          let b = self.registers[b];

          self.registers[out] = !(a && b);
        }
        Op::Set(id, _) => {
          self.registers[id] = immediates[id];
        }
        Op::Noop => {}
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{Op, Simulation};

  #[test]
  /// Test the Nand operation and ensure that it works as expected
  fn op_nand() {
    let mut simulation = Simulation {
      registers: vec![false, false, false],
      ops: vec![Op::Set(0, false), Op::Set(1, false), Op::Nand(0, 1, 2)],
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
}
