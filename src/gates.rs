use crate::simulation::{NandOp, Simulation};

// pub trait Gate {
//   fn add_to(
//     &self,
//     out: usize,
//     simulation: &mut Simulation,
//     sourcemap: bool,
//   ) -> usize;
// }

// struct AndGate(usize, usize);

// impl Gate for AndGate {
//   fn add_to(
//     &self,
//     out: usize,
//     simulation: &mut Simulation,
//     sourcemap: bool,
//   ) -> usize {
//     let nand = simulation.add_quiet_gate(Gate::Nand(self.0, self.1));
//     simulation.add_quiet_gate_with_out(Gate::Not(nand), out);

//     if sourcemap {
//       simulation.add_sourcemap("And".to_owned(), vec![self.0, self.1], out);
//     }

//     out
//   }
// }

pub enum Gate {
  Nand(usize, usize),
  And(usize, usize),
  Not(usize),
  Or(usize, usize),
  Nor(usize, usize),
  Xor(usize, usize),
}

impl Gate {
  pub fn add_to(
    &self,
    out: usize,
    simulation: &mut Simulation,
    sourcemap: bool,
  ) {
    match *self {
      Self::Nand(a, b) => simulation.add_op(NandOp(a, b, out)),
      Self::Not(a) => {
        simulation.add_quiet_gate_with_out(Gate::Nand(a, a), out);

        if sourcemap {
          simulation.add_sourcemap("Not".to_owned(), vec![a], out);
        }
      }
      Self::And(a, b) => {
        let nand = simulation.add_quiet_gate(Gate::Nand(a, b));
        simulation.add_quiet_gate_with_out(Gate::Not(nand), out);

        if sourcemap {
          simulation.add_sourcemap("And".to_owned(), vec![a, b], out);
        }
      }
      Self::Or(a, b) => {
        let nand_a = simulation.add_quiet_gate(Gate::Nand(a, a));
        let nand_b = simulation.add_quiet_gate(Gate::Nand(b, b));
        simulation.add_quiet_gate_with_out(Gate::Nand(nand_a, nand_b), out);

        if sourcemap {
          simulation.add_sourcemap("Or".to_owned(), vec![a, b], out);
        }
      }
      Self::Nor(a, b) => {
        let or = simulation.add_quiet_gate(Gate::Or(a, b));
        simulation.add_quiet_gate_with_out(Gate::Not(or), out);

        if sourcemap {
          simulation.add_sourcemap("Nor".to_owned(), vec![a, b], out);
        }
      }
      Self::Xor(a, b) => {
        let or = simulation.add_quiet_gate(Gate::Or(a, b));
        let nand = simulation.add_quiet_gate(Gate::Nand(a, b));
        simulation.add_quiet_gate_with_out(Gate::And(or, nand), out);

        if sourcemap {
          simulation.add_sourcemap("Xor".to_owned(), vec![a, b], out);
        }
      }
    }
  }
}
