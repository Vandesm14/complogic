use internment::Intern;
use wasmer::Value;

use crate::module::{Gate, Module};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct InputConnection {
  pub gate_id: u32,
  pub pin_id: u32,
  pub output_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OutputConnection {
  pub gate_id: u32,
  pub pin_id: u32,
  pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PinMap {
  pub outputs: Vec<OutputConnection>,
  pub inputs: Vec<InputConnection>,
}

impl PinMap {
  pub fn new() -> Self {
    Self {
      inputs: Vec::new(),
      outputs: Vec::new(),
    }
  }
}

#[derive(Debug, Default)]
pub struct Simulation {
  pub gate_count: u32,
  pub modules: Vec<Module>,
  pub pin_map: PinMap,
  pub gate_map: Vec<(Intern<String>, Intern<String>, Gate)>,
}

impl Simulation {
  pub fn new() -> Self {
    Self {
      gate_count: 0,
      modules: Vec::new(),
      pin_map: PinMap::new(),
      gate_map: Vec::new(),
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

  pub fn gate_index(
    &self,
    module_id: Intern<String>,
    gate_id: Intern<String>,
  ) -> Option<u32> {
    self
      .module(module_id)
      .and_then(|m| m.gates.iter().position(|g| g.id == gate_id))
      .map(|i| i as u32)
  }

  pub fn wasm(
    &mut self,
    module_id: Intern<String>,
  ) -> Option<(&wasmer::Instance, &mut wasmer::Store)> {
    self.module_mut(module_id).map(|m| m.wasm())
  }

  pub fn execute(
    &mut self,
    module_id: Intern<String>,
    gate_id: Intern<String>,
    inputs: u32,
  ) -> Option<u32> {
    let gate_index = self.gate_index(module_id, gate_id);
    if let Some(gate_index) = gate_index {
      if let Some((instance, store)) = self.wasm(module_id) {
        let entrypoint = instance
          .exports
          .get_function("gates")
          .expect("Function 'gates' not found in module");

        let result = entrypoint
          .call(
            store,
            &[
              Value::I32(0_i32),
              Value::I32(gate_index as i32),
              Value::I32(inputs as i32),
            ],
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
    } else {
      None
    }
  }
}
