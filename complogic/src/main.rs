use serde::Deserialize;
use wasmer::Value;

#[derive(Debug, Default)]
struct Simulation {
  store: wasmer::Store,
  modules: Vec<Module>,
}

impl Simulation {
  fn new() -> Self {
    Self {
      store: wasmer::Store::default(),
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
  ) {
    let gate_index = self.gate_index(module.as_ref(), gate.as_ref());
    if let Some(gate_index) = gate_index {
      if let Some(instance) = self.instance_mut(module) {
        let entrypoint = instance
          .exports
          .get_function("gates")
          .expect("Function 'gates' not found in module");

        entrypoint
          .call(
            &mut self.store,
            &[Value::I32(0), Value::I32(gate_index), Value::I32(inputs)],
          )
          .expect("Failed to call gate function");
      }
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

  fn instance_mut(
    &mut self,
    module: impl AsRef<str>,
  ) -> Option<&mut wasmer::Instance> {
    self.module_mut(module.as_ref()).map(|m| m.instance_mut())
  }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
struct ModuleConfig {
  id: String,
  name: String,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
struct Module {
  module: ModuleConfig,
  gates: Vec<Gate>,

  #[serde(skip)]
  instance: Option<wasmer::Instance>,
}

impl Module {
  fn with_instance(mut self, instance: wasmer::Instance) -> Self {
    self.instance = Some(instance);
    self
  }

  fn instance_mut(&mut self) -> &mut wasmer::Instance {
    self.instance.as_mut().expect("Module instance is not set")
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
  let mut store = wasmer::Store::default();
  let module = wasmer::Module::from_file(
    &store,
    "target/wasm32-unknown-unknown/debug/complogic_gates.wasm",
  )?;
  let instance =
    wasmer::Instance::new(&mut store, &module, &wasmer::Imports::new())?;

  let gates_module: Module = toml::from_str(
    &std::fs::read_to_string("complogic-gates/gates.toml").unwrap(),
  )
  .unwrap();

  let mut simulation = Simulation::new();
  simulation.add_module(gates_module.with_instance(instance));

  let gate_index = simulation
    .gate_index("std", "counter")
    .expect("Gate not found in directory");

  // let gates = instance.exports.get_function("gates")?;
  // let result = gates.call(
  //   &mut store,
  //   &[Value::I32(0), Value::I32(gate_index), Value::I32(0)],
  // )?;
  // assert_eq!(result[0], Value::I32(1));

  // let result = gates.call(
  //   &mut store,
  //   &[Value::I32(0), Value::I32(gate_index), Value::I32(0)],
  // )?;
  // assert_eq!(result[0], Value::I32(2));

  Ok(())
}
