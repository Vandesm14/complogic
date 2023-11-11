use crate::{gates::Gate, GateLike};

#[derive(Debug, Clone)]
pub struct NandOp(pub usize, pub usize, pub usize);
pub type Ops = Vec<NandOp>;

#[derive(Debug)]
pub struct SourceMap {
  pub name: String,
  pub inputs: Vec<usize>,
  pub outputs: Vec<usize>,
}

impl SourceMap {
  pub fn display(&self, simulation: &Simulation) {
    let inputs = self
      .inputs
      .iter()
      .map(|input| simulation.registers[*input])
      .collect::<Vec<_>>();

    let outputs = self
      .outputs
      .iter()
      .map(|output| simulation.registers[*output])
      .collect::<Vec<_>>();

    println!("{}: {:?} -> {:?}", self.name, inputs, outputs);
  }
}

#[derive(Debug)]
pub struct Simulation {
  /// Registers that note the inputs and outputs of logic gates
  pub registers: Vec<bool>,

  /// The ops to run on the registers
  pub ops: Ops,

  /// The number of immediate values to allocate when running the simulation
  pub immediate_count: usize,

  /// The sourcemaps for the simulation (maps registers to gates)
  pub soucrmaps: Vec<SourceMap>,
}

impl Simulation {
  /// Creates a new simulation
  pub fn new(immediate_count: usize) -> Self {
    Self {
      registers: vec![false; immediate_count],
      ops: vec![],
      immediate_count,
      soucrmaps: vec![],
    }
  }

  /// Runs the VM with the given immediates
  pub fn run(&mut self, immediates: &[bool]) {
    if immediates.len() != self.immediate_count {
      panic!(
        "Expected {} immediates, got {}",
        self.immediate_count,
        immediates.len()
      );
    }

    immediates.iter().enumerate().for_each(|(i, value)| {
      self.registers[i] = *value;
    });

    self.ops.iter().for_each(|op| match *op {
      NandOp(a, b, out) => {
        let a = self.registers[a];
        let b = self.registers[b];
        self.registers[out] = !(a && b);
      }
    });
  }

  /// Allocates a new register and returns its index
  pub fn alloc_one(&mut self) -> usize {
    self.registers.push(false);
    self.registers.len() - 1
  }

  /// Allocates n new registers and returns their indicies
  pub fn alloc(&mut self, n: usize) -> Vec<usize> {
    let mut out = vec![];

    for _ in 0..n {
      out.push(self.alloc_one());
    }

    out
  }

  /// Adds a gate to the simulation and returns the output register
  pub fn add_gate(&mut self, gate: &dyn GateLike) -> Vec<usize> {
    let outs = self.alloc(gate.out_count());
    gate.add_to(outs.clone(), self, true);

    outs
  }

  /// Adds a gate and uses an existing register as the output
  pub fn add_gate_with_out(&mut self, gate: Gate, outs: Vec<usize>) {
    gate.add_to(outs, self, true);
  }

  pub fn add_quiet_gate(&mut self, gate: Gate) -> Vec<usize> {
    let outs = self.alloc(gate.out_count());
    gate.add_to(outs.clone(), self, false);

    outs
  }

  pub fn add_quiet_gate_with_out(&mut self, gate: Gate, outs: Vec<usize>) {
    gate.add_to(outs, self, false);
  }

  pub fn add_op(&mut self, op: NandOp) {
    self.ops.push(op);
  }

  pub fn register(&self, index: usize) -> bool {
    self.registers[index]
  }

  pub fn add_sourcemap(
    &mut self,
    name: String,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
  ) {
    self.soucrmaps.push(SourceMap {
      name,
      inputs,
      outputs,
    });
  }
}
