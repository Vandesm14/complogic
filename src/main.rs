use complogic::{Gate, Simulation};

fn main() {
  let mut simulation = Simulation::new(2);
  let [r, s] = [0, 1];

  let q = simulation.alloc_one();
  let qn = simulation.alloc_one();

  simulation.add_sourcemap("Q".to_owned(), vec![], q);
  simulation.add_sourcemap("Qn".to_owned(), vec![], qn);

  simulation.add_gate_with_out(Gate::Nor(r, qn), q);
  simulation.add_gate_with_out(Gate::Nor(s, q), qn);

  simulation.run(&[true, false]);
  simulation.run(&[false, true]);

  simulation.soucrmaps.iter().for_each(|sourcemap| {
    sourcemap.display(&simulation);
  });
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

    let out = simulation.add_gate(Gate::And(a, b));

    simulation.run(&[true, true]);
    assert!(simulation.registers[out]);

    simulation.run(&[true, false]);
    assert!(!simulation.registers[out]);
  }

  #[test]
  fn add_gate_with_out() {
    let mut simulation = Simulation::new(0);

    let a = simulation.alloc_one();

    simulation.add_gate_with_out(Gate::Not(a), a);

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

    let out = simulation.add_gate(Gate::Or(a, b));

    simulation.run(&[true, false]);
    assert!(simulation.registers[out]);
  }

  #[test]
  fn or_gate_false() {
    let mut simulation = Simulation::new(2);
    let [a, b] = [0, 1];

    let out = simulation.add_gate(Gate::Or(a, b));

    simulation.run(&[false, false]);
    assert!(!simulation.registers[out]);
  }

  #[test]
  fn rs_nor_latch() {
    let mut simulation = Simulation::new(2);
    let [s, r] = [0, 1];

    let q = simulation.alloc_one();
    let qn = simulation.alloc_one();

    simulation.add_gate_with_out(Gate::Nor(r, qn), q);
    simulation.add_gate_with_out(Gate::Nor(s, q), qn);

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
