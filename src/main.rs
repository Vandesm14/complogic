use complogic::{Gate, GateLike, Simulation};

struct HalfAdder(usize, usize);

impl GateLike for HalfAdder {
  fn add_to(
    &self,
    outs: Vec<usize>,
    simulation: &mut Simulation,
    sourcemap: bool,
  ) {
    let [a, b] = [self.0, self.1];

    simulation.add_quiet_gate_with_out(&Gate::Xor(a, b), vec![outs[0]]);
    simulation.add_quiet_gate_with_out(&Gate::And(a, b), vec![outs[1]]);

    if sourcemap {
      simulation.add_sourcemap(
        "HalfAdder".to_owned(),
        vec![a, b],
        vec![outs[0]],
      );
    }
  }

  fn out_count(&self) -> usize {
    2
  }
}

struct FullAdder(usize, usize, usize);

impl GateLike for FullAdder {
  fn add_to(
    &self,
    outs: Vec<usize>,
    simulation: &mut Simulation,
    sourcemap: bool,
  ) {
    let [a, b, cin] = [self.0, self.1, self.2];
    let [sum, cout] = [outs[0], outs[1]];

    let half_adder_1 = simulation.add_gate(&HalfAdder(a, b));
    let half_adder_2 =
      simulation.add_gate_with_out(&HalfAdder(half_adder_1[0], cin), vec![sum]);

    simulation.add_quiet_gate_with_out(
      &Gate::Or(half_adder_2[1], half_adder_1[1]),
      vec![outs],
    )
  }

  fn out_count(&self) -> usize {
    3
  }
}

fn main() {
  let mut simulation = Simulation::new(2);
  let [a, b] = [0, 1];

  let gate = simulation.add_gate(&HalfAdder(a, b));
  let sum = gate[0];
  let carry = gate[1];

  simulation.run(&[false, false]);
  assert!(!simulation.registers[sum]);
  assert!(!simulation.registers[carry]);

  simulation.run(&[true, false]);
  assert!(simulation.registers[sum]);
  assert!(!simulation.registers[carry]);

  simulation.run(&[false, true]);
  assert!(simulation.registers[sum]);
  assert!(!simulation.registers[carry]);

  simulation.run(&[true, true]);
  assert!(!simulation.registers[sum]);
  assert!(simulation.registers[carry]);
}

#[cfg(test)]
mod tests {
  use complogic::NandOp;

  use super::*;

  #[test]
  fn op_nand() {
    let mut simulation = Simulation {
      registers: vec![false, false, false],
      ops: vec![NandOp(0, 1, 2)],
      immediate_count: 2,
      soucrmaps: vec![],
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
  fn add_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let out = *simulation.add_gate(&Gate::And(a, b)).first().unwrap();

    simulation.run(&[true, true]);
    assert!(simulation.registers[out]);

    simulation.run(&[true, false]);
    assert!(!simulation.registers[out]);
  }

  #[test]
  fn add_gate_with_out() {
    let mut simulation = Simulation::new(0);

    let a = simulation.alloc_one();

    simulation.add_gate_with_out(Gate::Not(a), vec![a]);

    simulation.run(&[]);
    assert!(simulation.registers[a]);

    simulation.run(&[]);
    assert!(!simulation.registers[a]);

    simulation.run(&[]);
    assert!(simulation.registers[a]);
  }

  #[test]
  fn or_gate() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let out = *simulation.add_gate(&Gate::Or(a, b)).first().unwrap();

    simulation.run(&[true, false]);
    assert!(simulation.registers[out]);
  }

  #[test]
  fn or_gate_false() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let out = *simulation.add_gate(&Gate::Or(a, b)).first().unwrap();

    simulation.run(&[false, false]);
    assert!(!simulation.registers[out]);
  }

  #[test]
  fn rs_nor_latch() {
    let mut simulation = Simulation::new(2);
    let [s, r] = [0, 1];

    let q = simulation.alloc_one();
    let qn = simulation.alloc_one();

    simulation.add_gate_with_out(Gate::Nor(r, qn), vec![q]);
    simulation.add_gate_with_out(Gate::Nor(s, q), vec![qn]);

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
}
