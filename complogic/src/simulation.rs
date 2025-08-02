use wasmer::Value;

use crate::module::Module;

#[derive(Debug, Default)]
pub struct Simulation {
  pub modules: Vec<Module>,
}

impl Simulation {
  pub fn new() -> Self {
    Self {
      modules: Vec::new(),
    }
  }

  pub fn add_module(&mut self, module: Module) {
    self.modules.push(module);
  }

  pub fn module(&self, id: impl AsRef<str>) -> Option<&Module> {
    let id = id.as_ref();
    self.modules.iter().find(|m| m.module.id == id)
  }

  pub fn module_mut(&mut self, id: impl AsRef<str>) -> Option<&mut Module> {
    let id = id.as_ref();
    self.modules.iter_mut().find(|m| m.module.id == id)
  }

  pub fn gate_index(
    &self,
    module: impl AsRef<str>,
    gate: impl AsRef<str>,
  ) -> Option<i32> {
    self
      .module(module.as_ref())
      .and_then(|m| m.gates.iter().position(|g| g.id == gate.as_ref()))
      .map(|i| i as i32)
  }

  pub fn wasm(
    &mut self,
    module: impl AsRef<str>,
  ) -> Option<(&wasmer::Instance, &mut wasmer::Store)> {
    self.module_mut(module.as_ref()).map(|m| m.wasm())
  }

  pub fn execute(
    &mut self,
    module: impl AsRef<str>,
    gate: impl AsRef<str>,
    inputs: i32,
  ) -> Option<i32> {
    let gate_index = self.gate_index(module.as_ref(), gate.as_ref());
    if let Some(gate_index) = gate_index {
      if let Some((instance, store)) = self.wasm(module) {
        let entrypoint = instance
          .exports
          .get_function("gates")
          .expect("Function 'gates' not found in module");

        let result = entrypoint
          .call(
            store,
            &[Value::I32(0), Value::I32(gate_index), Value::I32(inputs)],
          )
          .expect("Failed to call gate function");
        if let Value::I32(result) = result[0] {
          Some(result)
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
