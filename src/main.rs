use std::rc::Rc;

use complogic::{And, DLatch, Simulation};

fn main() {
  let mut simulation = Simulation::new(2);
  let [d, e] = [0, 1];

  let dlatch = Rc::new(DLatch {
    d,
    e,
    q: simulation.alloc(),
  });

  simulation.compile(vec![dlatch.clone()]);

  simulation.run(&[false, false]);
  println!("init: {}", simulation.registers[dlatch.q]);

  simulation.run(&[false, true]);
  println!("clk: {}", simulation.registers[dlatch.q]);

  simulation.run(&[true, false]);
  println!("set: {}", simulation.registers[dlatch.q]);

  simulation.run(&[true, true]);
  println!("clk + set: {}", simulation.registers[dlatch.q]);
}
