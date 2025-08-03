use internment::Intern;
use wasmer::Value;

use crate::module::{Gate, Module};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Connection {
  pub node_id: usize,
  pub pin_id: u32,
  pub output_id: usize,
}

impl Connection {
  pub fn new(node_id: usize, pin_id: u32, output_id: usize) -> Self {
    Self {
      node_id,
      pin_id,
      output_id,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PinMap {
  pub outputs: Vec<Connection>,
  pub inputs: Vec<Connection>,
}

impl PinMap {
  pub fn new() -> Self {
    Self {
      inputs: Vec::new(),
      outputs: Vec::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
  pub module_id: Intern<String>,
  pub gate_id: Intern<String>,
}

impl Node {
  pub fn new(module_id: Intern<String>, gate_id: Intern<String>) -> Self {
    Self { module_id, gate_id }
  }

  pub fn id(&self) -> (Intern<String>, Intern<String>) {
    (self.module_id, self.gate_id)
  }
}

#[derive(Debug, Default)]
pub struct Simulation {
  pub gate_count: u32,
  pub modules: Vec<Module>,
  pub outputs: Vec<bool>,
  pub nodes: Vec<Node>,
  pub pin_map: PinMap,
}

impl Simulation {
  pub fn new() -> Self {
    Self {
      gate_count: 0,
      modules: Vec::new(),
      outputs: Vec::new(),
      nodes: Vec::new(),
      pin_map: PinMap::new(),
    }
  }

  pub fn add_module(&mut self, module: Module) {
    self.modules.push(module);
  }

  pub fn module(&self, id: Intern<String>) -> Option<&Module> {
    self.modules.iter().find(|m| m.module.id == id)
  }

  pub fn module_mut(&mut self, id: Intern<String>) -> Option<&mut Module> {
    self.modules.iter_mut().find(|m| m.module.id == id)
  }

  pub fn add_gate(
    &mut self,
    module_id: Intern<String>,
    gate_id: Intern<String>,
  ) -> Option<u32> {
    if let Some(gate) = self.gate(module_id, gate_id) {
      let first_output_id = self.outputs.len();
      let node_id = self.nodes.len();

      for pin_id in 0..gate.outputs {
        self.outputs.push(false);

        self.pin_map.outputs.push(Connection::new(
          node_id,
          pin_id,
          first_output_id + pin_id as usize,
        ));
      }

      self.nodes.push(Node::new(module_id, gate_id));
    }

    None
  }

  pub fn gate(
    &self,
    module_id: Intern<String>,
    gate_id: Intern<String>,
  ) -> Option<&Gate> {
    self
      .module(module_id)
      .and_then(|m| m.gates.iter().find(|g| g.id == gate_id))
  }

  pub fn wasm(
    &mut self,
    module_id: Intern<String>,
  ) -> Option<(&wasmer::Instance, &mut wasmer::Store)> {
    self.module_mut(module_id).map(|m| m.wasm())
  }

  pub fn execute(
    &mut self,
    node_id: usize,
    module_id: Intern<String>,
    gate_id: Intern<String>,
    inputs: u32,
  ) -> Option<u32> {
    if let Some((instance, store)) = self.wasm(module_id) {
      let entrypoint = instance
        .exports
        .get_function(gate_id.as_str())
        .expect("Function 'gates' not found in module");

      let result = entrypoint
        .call(
          store,
          &[Value::I32(node_id as i32), Value::I32(inputs as i32)],
        )
        .expect("Failed to call gate function");
      if let Value::I32(result) = result[0] {
        Some(result as u32)
      } else {
        None
      }
    } else {
      None
    }
  }

  fn calculate_node_inputs(&self, node_id: usize) -> u32 {
    self
      .pin_map
      .inputs
      .iter()
      .filter(|c| c.node_id == node_id)
      .fold(0_u32, |pins, conn| {
        if let Some(output) = self.outputs.get(conn.output_id) {
          if *output {
            pins | (1 << conn.pin_id)
          } else {
            pins
          }
        } else {
          pins
        }
      })
  }

  fn update_node_outputs(&mut self, node_id: usize, outputs: u32) {
    self
      .pin_map
      .outputs
      .iter()
      .filter(|c| c.node_id == node_id)
      .for_each(|c| {
        if let Some(output) = self.outputs.get_mut(c.output_id) {
          *output = (outputs & (1 << c.pin_id)) != 0;
        }
      });
  }

  pub fn step(&mut self) {
    // First collect all the nodes we need to process to avoid borrowing issues
    let nodes: Vec<_> = self
      .nodes
      .iter()
      .enumerate()
      .map(|(id, node)| (id, node.module_id, node.gate_id))
      .collect();

    // Process each node
    for (node_id, module_id, gate_id) in nodes {
      // Calculate inputs for this specific node
      let input_pins = self.calculate_node_inputs(node_id);

      // Execute the gate and update outputs immediately
      if let Some(outputs) =
        self.execute(node_id, module_id, gate_id, input_pins)
      {
        self.update_node_outputs(node_id, outputs);
      }
    }
  }
}
