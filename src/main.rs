use std::rc::Rc;

use complogic::{And, Gate, Simulation, Xor};

#[derive(Debug)]
struct HalfAdder {
  a: usize,
  b: usize,
  s: usize,
  c: usize,
}

impl Gate for HalfAdder {
  fn create(&self, incrementer: &complogic::Incrementer) -> complogic::Ops {
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

    let mut ops: complogic::Ops = vec![];
    ops.extend(xor.create(incrementer));
    ops.extend(and.create(incrementer));

    ops
  }
}

fn main() {
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

  println!("S: {}", simulation.register(half_adder.s));
  println!("C: {}", simulation.register(half_adder.c));
  println!("Registers: {:?}", simulation.registers);
  println!("Ops: {:?}", simulation.ops);
}

#[cfg(test)]
mod tests {
  use complogic::{And, Incrementer, NandOp, Nor, Or};

  use super::*;

  #[test]
  fn op_nand() {
    let mut simulation = Simulation {
      registers: vec![false, false, false],
      ops: vec![NandOp(0, 1, 2)],
      immediate_count: 2,
      incrementer: Incrementer::set(2 - 1),
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

    let q = simulation.alloc();
    let qn = simulation.alloc();

    // simulation.add_gate_with_out(Gate::Nor(r, qn), q);
    // simulation.add_gate_with_out(Gate::Nor(s, q), qn);

    simulation.compile(vec![
      Rc::new(Nor {
        a: r,
        b: qn,
        out: q,
      }),
      Rc::new(Nor {
        a: s,
        b: q,
        out: qn,
      }),
    ]);

    // Reset the latch (due to the nature of logic, it starts as set when it's created)
    simulation.run(&[false, true]);

    simulation.run(&[false, false]);
    assert!(!simulation.registers[q]);
    assert!(simulation.registers[qn]);

    // FIXME: I think it's incorrect for it to need 2 ticks to set?
    simulation.run(&[true, false]);
    simulation.run(&[true, false]);
    assert!(simulation.registers[q]);
    assert!(!simulation.registers[qn]);

    simulation.run(&[false, true]);
    assert!(!simulation.registers[q]);
    assert!(simulation.registers[qn]);

    simulation.run(&[true, true]);
    assert!(!simulation.registers[q]);
    assert!(!simulation.registers[qn]);
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
}
