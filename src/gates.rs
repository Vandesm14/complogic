use crate::simulation::{NandOp, Simulation};

pub trait GateLike {
  fn add_to(
    &self,
    outs: Vec<usize>,
    simulation: &mut Simulation,
    sourcemap: bool,
  );

  fn out_count(&self) -> usize {
    1
  }
}

pub enum Gate {
  Nand(usize, usize),
  And(usize, usize),
  Not(usize),
  Or(usize, usize),
  Nor(usize, usize),
  Xor(usize, usize),
}

impl GateLike for Gate {
  fn add_to(
    &self,
    outs: Vec<usize>,
    simulation: &mut Simulation,
    sourcemap: bool,
  ) {
    let out = outs[0];

    match *self {
      Self::Nand(a, b) => simulation.add_op(NandOp(a, b, out)),
      Self::Not(a) => {
        simulation.add_quiet_gate_with_out(Gate::Nand(a, a), outs);

        if sourcemap {
          simulation.add_sourcemap("Not".to_owned(), vec![a], vec![out]);
        }
      }
      Self::And(a, b) => {
        let nand =
          *simulation.add_quiet_gate(Gate::Nand(a, b)).first().unwrap();
        simulation.add_quiet_gate_with_out(Gate::Not(nand), outs);

        if sourcemap {
          simulation.add_sourcemap("And".to_owned(), vec![a, b], vec![out]);
        }
      }
      Self::Or(a, b) => {
        let nand_a =
          *simulation.add_quiet_gate(Gate::Nand(a, a)).first().unwrap();
        let nand_b =
          *simulation.add_quiet_gate(Gate::Nand(b, b)).first().unwrap();
        simulation.add_quiet_gate_with_out(Gate::Nand(nand_a, nand_b), outs);

        if sourcemap {
          simulation.add_sourcemap("Or".to_owned(), vec![a, b], vec![out]);
        }
      }
      Self::Nor(a, b) => {
        let or = *simulation.add_quiet_gate(Gate::Or(a, b)).first().unwrap();
        simulation.add_quiet_gate_with_out(Gate::Not(or), outs);

        if sourcemap {
          simulation.add_sourcemap("Nor".to_owned(), vec![a, b], vec![out]);
        }
      }
      Self::Xor(a, b) => {
        let or = *simulation.add_quiet_gate(Gate::Or(a, b)).first().unwrap();
        let nand =
          *simulation.add_quiet_gate(Gate::Nand(a, b)).first().unwrap();
        simulation.add_quiet_gate_with_out(Gate::And(or, nand), outs);

        if sourcemap {
          simulation.add_sourcemap("Xor".to_owned(), vec![a, b], vec![out]);
        }
      }
    }
  }
}
