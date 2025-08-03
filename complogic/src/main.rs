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
  // let counter = Intern::from_ref("counter");
  // let and = Intern::from_ref("and");
  let identity = Intern::from_ref("identity");

  simulation.add_gate(std, identity);

  let output_id = simulation.outputs.len();
  simulation.outputs.push(true);
  simulation
    .pin_map
    .inputs
    .push(Connection::new(0, 0, output_id));

  println!("{:?}", simulation.outputs);
  simulation.step();
  println!("{:?}", simulation.outputs);
  simulation.step();
  println!("{:?}", simulation.outputs);

  *simulation.outputs.get_mut(output_id).unwrap() = false;
  simulation.step();
  println!("{:?}", simulation.outputs);

  // let result = simulation.execute(std, counter, 0);
  // assert_eq!(result, Some(1));

  // let result = simulation.execute(std, counter, 0);
  // assert_eq!(result, Some(2));

  // let result = simulation.execute(std, and, 0b00);
  // assert_eq!(result, Some(0));

  // let result = simulation.execute(std, and, 0b01);
  // assert_eq!(result, Some(0));

  // let result = simulation.execute(std, and, 0b10);
  // assert_eq!(result, Some(0));

  // let result = simulation.execute(std, and, 0b11);
  // assert_eq!(result, Some(1));

  // let result = simulation.execute(std, identity, 0b01);
  // assert_eq!(result, Some(1));

  // let result = simulation.execute(std, identity, 0b10);
  // assert_eq!(result, Some(2));

  Ok(())
}
