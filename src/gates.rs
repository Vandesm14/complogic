use crate::Incrementer;
use std::fmt::Debug;

// pub enum Gate {
//   Nand { a: usize, b: usize, out: usize },
//   And { a: usize, b: usize, out: usize },
//   Not { a: usize, out: usize },
//   Or { a: usize, b: usize, out: usize },
//   Nor { a: usize, b: usize, out: usize },
//   Xor { a: usize, b: usize, out: usize },
// }

// impl Gate {
//   pub fn create(&self, incrementer: Incrementer, ops: Ops) {
//     match *self {
//       Self::Nand { a, b, out } => {
//         ops.push(NandOp(a, b, out));
//       }
//       Self::Not { a, out } => {
//         Gate::Nand { a, b: a, out }.create(incrementer, ops);
//       }
//       Self::And { a, b, out } => {
//         let nand = Gate::Nand {
//           a,
//           b,
//           out: incrementer.next(),
//         };
//         Gate::Not { a: , out };
//       }
//       Self::Or { a, b, out } => {
//         // let nand_a = simulation.add_quiet_gate(Gate::Nand(a, a));
//         // let nand_b = simulation.add_quiet_gate(Gate::Nand(b, b));
//         // simulation.add_quiet_gate_with_out(Gate::Nand(nand_a, nand_b), out);
//         todo!()
//       }
//       Self::Nor { a, b, out } => {
//         // let or = simulation.add_quiet_gate(Gate::Or(a, b));
//         // simulation.add_quiet_gate_with_out(Gate::Not(or), out);
//         todo!()
//       }
//       Self::Xor { a, b, out } => {
//         // let or = simulation.add_quiet_gate(Gate::Or(a, b));
//         // let nand = simulation.add_quiet_gate(Gate::Nand(a, b));
//         // simulation.add_quiet_gate_with_out(Gate::And(or, nand), out);
//         todo!()
//       }
//     }
//   }
// }

type Gates = Vec<Box<dyn Gate>>;

pub trait Gate: Debug {
  fn create(&self, incrementer: &Incrementer) -> Gates;
}

#[derive(Debug)]
pub struct Nand {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Nand {
  fn create(&self, _: &Incrementer) -> Gates {
    vec![Box::new(Nand {
      a: self.a,
      b: self.a,
      out: self.out,
    })]
  }
}

#[derive(Debug)]
pub struct Not {
  pub a: usize,
  pub out: usize,
}

impl Gate for Not {
  fn create(&self, _: &Incrementer) -> Gates {
    vec![Box::new(Nand {
      a: self.a,
      b: self.a,
      out: self.out,
    })]
  }
}

#[derive(Debug)]
pub struct And {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for And {
  fn create(&self, incrementer: &Incrementer) -> Gates {
    let nand = Nand {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let not = Not {
      a: nand.out,
      out: self.out,
    };

    let mut gates: Gates = vec![Box::new(nand)];
    gates.extend(not.create(incrementer));

    gates
  }
}

#[derive(Debug)]
pub struct Or {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Or {
  fn create(&self, incrementer: &Incrementer) -> Gates {
    let nand_a = Nand {
      a: self.a,
      b: self.a,
      out: incrementer.next(),
    };
    let nand_b = Nand {
      a: self.b,
      b: self.b,
      out: incrementer.next(),
    };
    let nand = Nand {
      a: nand_a.out,
      b: nand_b.out,
      out: self.out,
    };

    vec![Box::new(nand_a), Box::new(nand_b), Box::new(nand)]
  }
}

#[derive(Debug)]
pub struct Nor {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Nor {
  fn create(&self, incrementer: &Incrementer) -> Gates {
    let or = Or {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let not = Not {
      a: or.out,
      out: self.out,
    };

    let mut gates: Gates = vec![];
    gates.extend(or.create(incrementer));
    gates.extend(not.create(incrementer));

    gates
  }
}
