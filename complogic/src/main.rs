use complogic::{module::Module, simulation::Simulation};

fn main() -> anyhow::Result<()> {
  let std = Module::from_file("complogic-gates/gates.toml");

  let mut simulation = Simulation::new();
  simulation.add_module(std);

  let result = simulation.execute("std", "counter", 0);
  assert_eq!(result, Some(1));

  let result = simulation.execute("std", "counter", 0);
  assert_eq!(result, Some(2));

  Ok(())
}
