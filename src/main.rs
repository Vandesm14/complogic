use complogic::Simulation;

fn main() {
  let mut simulation = Simulation::new(2);
  assert_eq!(simulation.registers.len(), 2);

  simulation.compile(vec![]);
  assert_eq!(simulation.registers.len(), 2);

  assert_eq!(simulation.alloc(), 2);
  assert_eq!(simulation.alloc(), 3);

  simulation.compile(vec![]);
  assert_eq!(simulation.registers.len(), 4);
}
