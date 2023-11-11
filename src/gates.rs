use crate::{Incrementer, NandOp, Ops};
use std::{fmt::Debug, rc::Rc};

pub trait Gate: Debug {
  fn create(&self, incrementer: &Incrementer) -> Ops;
}

#[derive(Debug)]
pub struct Nand {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Nand {
  fn create(&self, _: &Incrementer) -> Ops {
    vec![NandOp(self.a, self.b, self.out)]
  }
}

#[derive(Debug)]
pub struct Not {
  pub a: usize,
  pub out: usize,
}

impl Gate for Not {
  fn create(&self, _: &Incrementer) -> Ops {
    vec![NandOp(self.a, self.a, self.out)]
  }
}

#[derive(Debug)]
pub struct And {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for And {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let nand = Nand {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let not = Not {
      a: nand.out,
      out: self.out,
    };

    let mut ops: Ops = vec![];
    ops.extend(nand.create(incrementer));
    ops.extend(not.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct Or {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Or {
  fn create(&self, incrementer: &Incrementer) -> Ops {
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

    let mut ops: Ops = vec![];
    ops.extend(nand_a.create(incrementer));
    ops.extend(nand_b.create(incrementer));
    ops.extend(nand.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct Nor {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Nor {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let or = Or {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let not = Not {
      a: or.out,
      out: self.out,
    };

    let mut ops: Ops = vec![];
    ops.extend(or.create(incrementer));
    ops.extend(not.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct Xor {
  pub a: usize,
  pub b: usize,
  pub out: usize,
}

impl Gate for Xor {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let or = Or {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let nand = Nand {
      a: self.a,
      b: self.b,
      out: incrementer.next(),
    };
    let and = And {
      a: nand.out,
      b: nand.out,
      out: self.out,
    };

    let mut ops: Ops = vec![];
    ops.extend(or.create(incrementer));
    ops.extend(nand.create(incrementer));
    ops.extend(and.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct RSLatch {
  pub s: usize,
  pub r: usize,
  pub q: usize,
}

impl Gate for RSLatch {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let q_patch = incrementer.next();
    let qn_patch = incrementer.next();

    let nor_1 = Rc::new(Nor {
      a: self.s,
      b: qn_patch,
      out: q_patch,
    });
    let nor_2 = Rc::new(Nor {
      a: self.r,
      b: q_patch,
      out: qn_patch,
    });

    let or_q = Rc::new(Or {
      a: qn_patch,
      b: qn_patch,
      out: self.q,
    });

    let mut ops: Ops = vec![];
    ops.extend(nor_1.create(incrementer));
    ops.extend(nor_2.create(incrementer));
    ops.extend(or_q.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct DLatch {
  pub d: usize,
  pub e: usize,
  pub q: usize,
}

impl Gate for DLatch {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let not = Rc::new(Not {
      a: self.d,
      out: incrementer.next(),
    });

    let and_1 = Rc::new(And {
      a: not.out,
      b: self.e,
      out: incrementer.next(),
    });
    let and_2 = Rc::new(And {
      a: self.e,
      b: self.d,
      out: incrementer.next(),
    });

    // FIXME: It's strange that I have to flip which gates go into the S and R of the latch.
    // The latch itself works fine, see the test for it, so I'm not sure why it's backwards here.
    let rs_latch = RSLatch {
      s: and_2.out,
      r: and_1.out,
      q: self.q,
    };

    let mut ops: Ops = vec![];
    ops.extend(not.create(incrementer));
    ops.extend(and_1.create(incrementer));
    ops.extend(and_2.create(incrementer));
    ops.extend(rs_latch.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct HalfAdder {
  pub a: usize,
  pub b: usize,
  pub s: usize,
  pub c: usize,
}

impl Gate for HalfAdder {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let xor = Rc::new(Xor {
      a: self.a,
      b: self.b,
      out: self.s,
    });
    let and = Rc::new(And {
      a: self.a,
      b: self.b,
      out: self.c,
    });

    let mut ops: Ops = vec![];
    ops.extend(xor.create(incrementer));
    ops.extend(and.create(incrementer));

    ops
  }
}

#[derive(Debug)]
pub struct FullAdder {
  pub a: usize,
  pub b: usize,
  pub cin: usize,
  pub s: usize,
  pub cout: usize,
}

impl Gate for FullAdder {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let half_adder_1 = HalfAdder {
      a: self.a,
      b: self.b,
      s: incrementer.next(),
      c: incrementer.next(),
    };
    let half_adder_2 = HalfAdder {
      a: half_adder_1.s,
      b: self.cin,
      s: self.s,
      c: incrementer.next(),
    };
    let or = Or {
      a: half_adder_1.c,
      b: half_adder_2.c,
      out: self.cout,
    };

    let mut ops: Ops = vec![];
    ops.extend(half_adder_1.create(incrementer));
    ops.extend(half_adder_2.create(incrementer));
    ops.extend(or.create(incrementer));

    ops
  }
}

#[derive(Debug)]

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

impl Gate for FourBitAdder {
  fn create(&self, incrementer: &Incrementer) -> Ops {
    let full_adder_1 = Rc::new(FullAdder {
      a: self.a1,
      b: self.b1,
      cin: incrementer.next(),
      s: self.s1,
      cout: incrementer.next(),
    });
    let full_adder_2 = Rc::new(FullAdder {
      a: self.a2,
      b: self.b2,
      cin: full_adder_1.cout,
      s: self.s2,
      cout: incrementer.next(),
    });
    let full_adder_3 = Rc::new(FullAdder {
      a: self.a3,
      b: self.b3,
      cin: full_adder_2.cout,
      s: self.s3,
      cout: incrementer.next(),
    });
    let full_adder_4 = Rc::new(FullAdder {
      a: self.a4,
      b: self.b4,
      cin: full_adder_3.cout,
      s: self.s4,
      cout: self.cout,
    });

    let mut ops: Ops = vec![];
    ops.extend(full_adder_1.create(incrementer));
    ops.extend(full_adder_2.create(incrementer));
    ops.extend(full_adder_3.create(incrementer));
    ops.extend(full_adder_4.create(incrementer));

    ops
  }
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;

  use super::*;
  use crate::Simulation;

  #[test]
  fn and_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let and = Rc::new(And {
      a,
      b,
      out: simulation.alloc(),
    });

    simulation.compile(vec![and.clone()]);

    simulation.run(&[true, true]);
    assert!(simulation.registers[and.out]);

    simulation.run(&[true, false]);
    assert!(!simulation.registers[and.out]);
  }

  #[test]
  fn or_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let or = Rc::new(Or {
      a,
      b,
      out: simulation.alloc(),
    });

    simulation.compile(vec![or.clone()]);

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

    let rslatch = Rc::new(RSLatch {
      s,
      r,
      q: simulation.alloc(),
    });

    simulation.compile(vec![rslatch.clone()]);

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

    let dlatch = Rc::new(DLatch {
      d,
      e,
      q: simulation.alloc(),
    });

    simulation.compile(vec![dlatch.clone()]);

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

    let half_adder = Rc::new(HalfAdder { a, b, s, c });

    simulation.compile(vec![half_adder.clone()]);

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

    let full_adder = Rc::new(FullAdder { a, b, cin, s, cout });

    simulation.compile(vec![full_adder.clone()]);

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

    simulation.incrementer.next_n(5);
    let [s5, s4, s3, s2, s1] = [8, 9, 10, 11, 12];

    let four_bit_adder = Rc::new(FourBitAdder {
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
    });

    simulation.compile(vec![four_bit_adder.clone()]);

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
