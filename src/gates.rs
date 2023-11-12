use crate::{Incrementer, NandOp, Ops};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Nand {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Not {
  pub a: usize,
  pub out: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct And {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Or {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Nor {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Xor {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RSLatch {
  pub s: usize,
  pub r: usize,
  pub q: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DLatch {
  pub d: usize,
  pub e: usize,
  pub q: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HalfAdder {
  pub a: usize,
  pub b: usize,
  pub s: usize,
  pub c: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FullAdder {
  pub a: usize,
  pub b: usize,
  pub cin: usize,
  pub s: usize,
  pub cout: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]

pub struct FourBitAdder {
  pub a1: usize,
  pub a2: usize,
  pub a3: usize,
  pub a4: usize,
  pub b1: usize,
  pub b2: usize,
  pub b3: usize,
  pub b4: usize,
  pub s1: usize,
  pub s2: usize,
  pub s3: usize,
  pub s4: usize,
  pub cout: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Gate {
  Nand(Nand),
  Not(Not),
  And(And),
  Or(Or),
  Nor(Nor),
  Xor(Xor),
  RSLatch(RSLatch),
  DLatch(DLatch),
  HalfAdder(HalfAdder),
  FullAdder(FullAdder),
  FourBitAdder(FourBitAdder),
}

impl From<Nand> for Gate {
  fn from(nand: Nand) -> Self {
    Self::Nand(nand)
  }
}

impl From<Not> for Gate {
  fn from(not: Not) -> Self {
    Self::Not(not)
  }
}

impl From<And> for Gate {
  fn from(and: And) -> Self {
    Self::And(and)
  }
}

impl From<Or> for Gate {
  fn from(or: Or) -> Self {
    Self::Or(or)
  }
}

impl From<Nor> for Gate {
  fn from(nor: Nor) -> Self {
    Self::Nor(nor)
  }
}

impl From<Xor> for Gate {
  fn from(xor: Xor) -> Self {
    Self::Xor(xor)
  }
}

impl From<RSLatch> for Gate {
  fn from(rs_latch: RSLatch) -> Self {
    Self::RSLatch(rs_latch)
  }
}

impl From<DLatch> for Gate {
  fn from(d_latch: DLatch) -> Self {
    Self::DLatch(d_latch)
  }
}

impl From<HalfAdder> for Gate {
  fn from(half_adder: HalfAdder) -> Self {
    Self::HalfAdder(half_adder)
  }
}

impl From<FullAdder> for Gate {
  fn from(full_adder: FullAdder) -> Self {
    Self::FullAdder(full_adder)
  }
}

impl From<FourBitAdder> for Gate {
  fn from(four_bit_adder: FourBitAdder) -> Self {
    Self::FourBitAdder(four_bit_adder)
  }
}

impl Gate {
  pub fn create(&self, incrementer: &mut Incrementer) -> Ops {
    match self {
      Gate::Nand(nand) => {
        vec![NandOp(nand.a, nand.b, nand.out)]
      }
      Gate::Not(not) => {
        vec![NandOp(not.a, not.a, not.out)]
      }
      Gate::And(and) => {
        let nand = Nand {
          a: and.a,
          b: and.b,
          out: incrementer.next(),
        };
        let not = Not {
          a: nand.out,
          out: and.out,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(nand).create(incrementer));
        ops.extend(Gate::from(not).create(incrementer));

        ops
      }
      Gate::Or(or) => {
        let nand_a = Nand {
          a: or.a,
          b: or.a,
          out: incrementer.next(),
        };
        let nand_b = Nand {
          a: or.b,
          b: or.b,
          out: incrementer.next(),
        };
        let nand = Nand {
          a: nand_a.out,
          b: nand_b.out,
          out: or.out,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(nand_a).create(incrementer));
        ops.extend(Gate::from(nand_b).create(incrementer));
        ops.extend(Gate::from(nand).create(incrementer));

        ops
      }
      Gate::Nor(nor) => {
        let or = Or {
          a: nor.a,
          b: nor.b,
          out: incrementer.next(),
        };
        let not = Not {
          a: or.out,
          out: nor.out,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(or).create(incrementer));
        ops.extend(Gate::from(not).create(incrementer));

        ops
      }
      Gate::Xor(xor) => {
        let or = Or {
          a: xor.a,
          b: xor.b,
          out: incrementer.next(),
        };
        let nand = Nand {
          a: xor.a,
          b: xor.b,
          out: incrementer.next(),
        };
        let and = And {
          a: or.out,
          b: nand.out,
          out: xor.out,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(or).create(incrementer));
        ops.extend(Gate::from(nand).create(incrementer));
        ops.extend(Gate::from(and).create(incrementer));

        ops
      }
      Gate::RSLatch(rs_latch) => {
        let q_patch = incrementer.next();
        let qn_patch = incrementer.next();

        let nor_1 = Nor {
          a: rs_latch.s,
          b: qn_patch,
          out: q_patch,
        };
        let nor_2 = Nor {
          a: rs_latch.r,
          b: q_patch,
          out: qn_patch,
        };

        let or_q = Or {
          a: qn_patch,
          b: qn_patch,
          out: rs_latch.q,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(nor_1).create(incrementer));
        ops.extend(Gate::from(nor_2).create(incrementer));
        ops.extend(Gate::from(or_q).create(incrementer));

        ops
      }
      Gate::DLatch(d_latch) => {
        let not = Not {
          a: d_latch.d,
          out: incrementer.next(),
        };

        let and_1 = And {
          a: not.out,
          b: d_latch.e,
          out: incrementer.next(),
        };
        let and_2 = And {
          a: d_latch.e,
          b: d_latch.d,
          out: incrementer.next(),
        };

        // FIXME: It's strange that I have to flip which gates go into the S and R of the latch.
        // The latch itself works fine, see the test for it, so I'm not sure why it's backwards here.
        let rs_latch = RSLatch {
          s: and_2.out,
          r: and_1.out,
          q: d_latch.q,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(not).create(incrementer));
        ops.extend(Gate::from(and_1).create(incrementer));
        ops.extend(Gate::from(and_2).create(incrementer));
        ops.extend(Gate::from(rs_latch).create(incrementer));

        ops
      }
      Gate::HalfAdder(half_adder) => {
        let xor = Xor {
          a: half_adder.a,
          b: half_adder.b,
          out: half_adder.s,
        };
        let and = And {
          a: half_adder.a,
          b: half_adder.b,
          out: half_adder.c,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(xor).create(incrementer));
        ops.extend(Gate::from(and).create(incrementer));

        ops
      }
      Gate::FullAdder(full_adder) => {
        let half_adder_1 = HalfAdder {
          a: full_adder.a,
          b: full_adder.b,
          s: incrementer.next(),
          c: incrementer.next(),
        };
        let half_adder_2 = HalfAdder {
          a: half_adder_1.s,
          b: full_adder.cin,
          s: full_adder.s,
          c: incrementer.next(),
        };
        let or = Or {
          a: half_adder_1.c,
          b: half_adder_2.c,
          out: full_adder.cout,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(half_adder_1).create(incrementer));
        ops.extend(Gate::from(half_adder_2).create(incrementer));
        ops.extend(Gate::from(or).create(incrementer));

        ops
      }
      Gate::FourBitAdder(four_bit_adder) => {
        let full_adder_1 = FullAdder {
          a: four_bit_adder.a1,
          b: four_bit_adder.b1,
          cin: incrementer.next(),
          s: four_bit_adder.s1,
          cout: incrementer.next(),
        };
        let full_adder_2 = FullAdder {
          a: four_bit_adder.a2,
          b: four_bit_adder.b2,
          cin: full_adder_1.cout,
          s: four_bit_adder.s2,
          cout: incrementer.next(),
        };
        let full_adder_3 = FullAdder {
          a: four_bit_adder.a3,
          b: four_bit_adder.b3,
          cin: full_adder_2.cout,
          s: four_bit_adder.s3,
          cout: incrementer.next(),
        };
        let full_adder_4 = FullAdder {
          a: four_bit_adder.a4,
          b: four_bit_adder.b4,
          cin: full_adder_3.cout,
          s: four_bit_adder.s4,
          cout: four_bit_adder.cout,
        };

        let mut ops: Ops = vec![];
        ops.extend(Gate::from(full_adder_1).create(incrementer));
        ops.extend(Gate::from(full_adder_2).create(incrementer));
        ops.extend(Gate::from(full_adder_3).create(incrementer));
        ops.extend(Gate::from(full_adder_4).create(incrementer));

        ops
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Simulation;

  #[test]
  fn and_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let and = And {
      a,
      b,
      out: simulation.alloc(),
    };

    simulation.compile(vec![&Gate::from(and)]);

    simulation.run(&[true, true]);
    assert!(simulation.registers[and.out]);

    simulation.run(&[true, false]);
    assert!(!simulation.registers[and.out]);
  }

  #[test]
  fn or_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let or = Or {
      a,
      b,
      out: simulation.alloc(),
    };

    simulation.compile(vec![&Gate::from(or)]);

    simulation.run(&[false, false]);
    assert!(!simulation.registers[or.out]);

    simulation.run(&[false, true]);
    assert!(simulation.registers[or.out]);

    simulation.run(&[true, false]);
    assert!(simulation.registers[or.out]);

    simulation.run(&[true, true]);
    assert!(simulation.registers[or.out]);
  }

  #[test]
  fn rs_nor_latch() {
    let mut simulation = Simulation::new(2);
    let [s, r] = [0, 1];

    let rslatch = RSLatch {
      s,
      r,
      q: simulation.alloc(),
    };

    simulation.compile(vec![&Gate::from(rslatch)]);

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

  #[test]
  fn dlatch() {
    let mut simulation = Simulation::new(2);
    let [d, e] = [0, 1];

    let dlatch = DLatch {
      d,
      e,
      q: simulation.alloc(),
    };

    simulation.compile(vec![&Gate::from(dlatch)]);

    simulation.run(&[false, false]);
    assert!(!simulation.registers[dlatch.q]);

    simulation.run(&[false, true]);
    assert!(!simulation.registers[dlatch.q]);

    simulation.run(&[true, false]);
    assert!(!simulation.registers[dlatch.q]);

    simulation.run(&[true, true]);
    assert!(simulation.registers[dlatch.q]);
  }

  #[test]
  fn half_adder() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let s = simulation.alloc();
    let c = simulation.alloc();

    let half_adder = HalfAdder { a, b, s, c };

    simulation.compile(vec![&Gate::from(half_adder)]);

    simulation.run(&[false, false]);
    assert!(!simulation.registers[half_adder.s]);
    assert!(!simulation.registers[half_adder.c]);

    simulation.run(&[false, true]);
    assert!(simulation.registers[half_adder.s]);
    assert!(!simulation.registers[half_adder.c]);

    simulation.run(&[true, false]);
    assert!(simulation.registers[half_adder.s]);
    assert!(!simulation.registers[half_adder.c]);

    simulation.run(&[true, true]);
    assert!(!simulation.registers[half_adder.s]);
    assert!(simulation.registers[half_adder.c]);
  }

  #[test]
  fn full_adder() {
    let mut simulation = Simulation::new(3);
    let [a, b, cin] = [0, 1, 2];

    let s = simulation.alloc();
    let cout = simulation.alloc();

    let full_adder = FullAdder { a, b, cin, s, cout };

    simulation.compile(vec![&Gate::from(full_adder)]);

    simulation.run(&[false, false, false]);
    assert!(!simulation.registers[full_adder.s]);
    assert!(!simulation.registers[full_adder.cout]);

    simulation.run(&[false, false, true]);
    assert!(simulation.registers[full_adder.s]);
    assert!(!simulation.registers[full_adder.cout]);

    simulation.run(&[false, true, false]);
    assert!(simulation.registers[full_adder.s]);
    assert!(!simulation.registers[full_adder.cout]);

    simulation.run(&[false, true, true]);
    assert!(!simulation.registers[full_adder.s]);
    assert!(simulation.registers[full_adder.cout]);

    simulation.run(&[true, false, false]);
    assert!(simulation.registers[full_adder.s]);
    assert!(!simulation.registers[full_adder.cout]);

    simulation.run(&[true, false, true]);
    assert!(!simulation.registers[full_adder.s]);
    assert!(simulation.registers[full_adder.cout]);

    simulation.run(&[true, true, false]);
    assert!(!simulation.registers[full_adder.s]);
    assert!(simulation.registers[full_adder.cout]);

    simulation.run(&[true, true, true]);
    assert!(simulation.registers[full_adder.s]);
    assert!(simulation.registers[full_adder.cout]);
  }

  fn number_to_bin_vec(number: usize, size: usize) -> Vec<bool> {
    let mut vec = vec![];
    let mut number = number;
    while number > 0 {
      vec.push(number % 2 == 1);
      number /= 2;
    }

    while vec.len() < size {
      vec.push(false);
    }

    vec.reverse();
    vec
  }

  #[test]
  fn four_bit_adder() {
    let mut simulation = Simulation::new(8);
    let [a4, a3, a2, a1, b4, b3, b2, b1] = [0, 1, 2, 3, 4, 5, 6, 7];

    simulation.incrementer.skip(5);
    let [s5, s4, s3, s2, s1] = [8, 9, 10, 11, 12];

    let four_bit_adder = FourBitAdder {
      a1,
      a2,
      a3,
      a4,
      b1,
      b2,
      b3,
      b4,
      s1,
      s2,
      s3,
      s4,
      cout: s5,
    };

    simulation.compile(vec![&Gate::from(four_bit_adder)]);

    for a in 0..0b1111 {
      let bin_a = number_to_bin_vec(a, 4);

      for b in 0..0b1111 {
        let bin_b = number_to_bin_vec(b, 4);

        let mut input = vec![];
        input.extend(bin_a.clone());
        input.extend(bin_b);

        simulation.run(&input);

        let bin_s = number_to_bin_vec(a + b, 5);
        assert_eq!(bin_s, &simulation.registers[s5..=s1]);
      }
    }
  }
}
