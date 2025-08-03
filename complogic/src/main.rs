use complogic::{
  module::Module,
  simulation::{Connection, Simulation},
};
use internment::Intern;

fn main() -> anyhow::Result<()> {
  let std = Module::from_file("complogic-gates/gates.toml");

  let mut simulation = Simulation::new();
  simulation.add_module(std);

  // simulation.add_gate()

  let std = Intern::from_ref("std");
  let and = Intern::from_ref("and");
  let not = Intern::from_ref("not");

  simulation.add_gate(std, not);
  simulation.add_gate(std, and);

  simulation.pin_map.inputs.push(Connection::new(1, 0, 0));
  simulation.pin_map.inputs.push(Connection::new(1, 1, 0));

  println!("{:?}", simulation.outputs);
  simulation.step();
  println!("{:?}", simulation.outputs);
  simulation.step();
  println!("{:?}", simulation.outputs);
  simulation.step();
  println!("{:?}", simulation.outputs);

  Ok(())
}
