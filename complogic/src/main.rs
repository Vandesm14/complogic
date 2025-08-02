use std::path::Path;

use serde::Deserialize;
use wasmer::Value;

#[derive(Debug, Default)]
struct Simulation {
  modules: Vec<Module>,
}

impl Simulation {
  fn new() -> Self {
    Self {
      modules: Vec::new(),
    }
  }

  fn add_module(&mut self, module: Module) {
    self.modules.push(module);
  }

  fn execute(
    &mut self,
    module: impl AsRef<str>,
    gate: impl AsRef<str>,
    inputs: i32,
  ) -> Option<i32> {
    let gate_index = self.gate_index(module.as_ref(), gate.as_ref());
    if let Some(gate_index) = gate_index {
      if let Some((instance, store)) = self.get_wasm(module) {
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

  fn module(&self, id: impl AsRef<str>) -> Option<&Module> {
    let id = id.as_ref();
    self.modules.iter().find(|m| m.module.id == id)
  }

  fn gate(
    &self,
    module: impl AsRef<str>,
    gate: impl AsRef<str>,
  ) -> Option<&Gate> {
    self
      .module(module.as_ref())
      .and_then(|m| m.gate(gate.as_ref()))
  }

  fn gate_index(
    &self,
    module: impl AsRef<str>,
    gate: impl AsRef<str>,
  ) -> Option<i32> {
    self
      .module(module.as_ref())
      .and_then(|m| m.gates.iter().position(|g| g.id == gate.as_ref()))
      .map(|i| i as i32)
  }

  fn module_mut(&mut self, id: impl AsRef<str>) -> Option<&mut Module> {
    let id = id.as_ref();
    self.modules.iter_mut().find(|m| m.module.id == id)
  }

  fn get_wasm(
    &mut self,
    module: impl AsRef<str>,
  ) -> Option<(&wasmer::Instance, &mut wasmer::Store)> {
    self.module_mut(module.as_ref()).map(|m| m.get_wasm())
  }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
struct ModuleConfig {
  id: String,
  name: String,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
struct Module {
  module: ModuleConfig,
  gates: Vec<Gate>,

  #[serde(skip)]
  instance: Option<wasmer::Instance>,
  #[serde(skip)]
  store: Option<wasmer::Store>,
}

impl Module {
  fn include(wasm_path: impl AsRef<Path>, toml_path: impl AsRef<Path>) -> Self {
    let mut store = wasmer::Store::default();
    let wasm_module = wasmer::Module::from_file(&store, wasm_path)
      .expect("Failed to load module");
    let instance =
      wasmer::Instance::new(&mut store, &wasm_module, &wasmer::Imports::new())
        .expect("Failed to instantiate module");
    let module: Module = toml::from_str(
      &std::fs::read_to_string(toml_path).expect("Failed to read module file"),
    )
    .expect("Failed to parse toml file");

    module.with_instance(instance).with_store(store)
  }

  fn with_instance(mut self, instance: wasmer::Instance) -> Self {
    self.instance = Some(instance);
    self
  }

  fn with_store(mut self, store: wasmer::Store) -> Self {
    self.store = Some(store);
    self
  }

  fn get_wasm(&mut self) -> (&wasmer::Instance, &mut wasmer::Store) {
    let instance = self.instance.as_ref().expect("Module instance not set");
    let store = self.store.as_mut().expect("Module store not set");
    (instance, store)
  }

  fn gate(&self, id: impl AsRef<str>) -> Option<&Gate> {
    let id = id.as_ref();
    self.gates.iter().find(|g| g.id == id)
  }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
struct Gate {
  id: String,
  name: String,
  description: String,
  inputs: i32,
  outputs: i32,
}

fn main() -> anyhow::Result<()> {
  let std = Module::include(
    "target/wasm32-unknown-unknown/debug/complogic_gates.wasm",
    "complogic-gates/gates.toml",
  );
  let std2 = Module::include(
    "target/wasm32-unknown-unknown/debug/complogic_gates.wasm",
    "complogic-gates/gates2.toml",
  );

  let mut simulation = Simulation::new();
  simulation.add_module(std);
  simulation.add_module(std2);

  let result = simulation.execute("std", "counter", 0);
  assert_eq!(result, Some(1));

  let result = simulation.execute("std2", "counter", 0);
  assert_eq!(result, Some(1));

  let result = simulation.execute("std", "counter", 0);
  assert_eq!(result, Some(2));

  let result = simulation.execute("std2", "counter", 0);
  assert_eq!(result, Some(2));

  Ok(())
}
