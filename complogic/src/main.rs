use complogic::{module::Module, simulation::Simulation};
use internment::Intern;

fn main() -> anyhow::Result<()> {
  let std = Module::from_file("complogic-gates/gates.toml");

  let mut simulation = Simulation::new();
  simulation.add_module(std);

  // simulation.add_gate()

  let std = Intern::from_ref("std");
  let counter = Intern::from_ref("counter");
  let and = Intern::from_ref("and");
  let identity = Intern::from_ref("identity");

  let result = simulation.execute(std, counter, 0);
  assert_eq!(result, Some(1));

  let result = simulation.execute(std, counter, 0);
  assert_eq!(result, Some(2));

  let result = simulation.execute(std, and, 0b00);
  assert_eq!(result, Some(0));

  let result = simulation.execute(std, and, 0b01);
  assert_eq!(result, Some(0));

  let result = simulation.execute(std, and, 0b10);
  assert_eq!(result, Some(0));

  let result = simulation.execute(std, and, 0b11);
  assert_eq!(result, Some(1));

  let result = simulation.execute(std, identity, 0b01);
  assert_eq!(result, Some(1));

  let result = simulation.execute(std, identity, 0b10);
  assert_eq!(result, Some(2));

  Ok(())
}
