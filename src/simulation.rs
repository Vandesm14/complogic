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
