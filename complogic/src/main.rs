use serde::Deserialize;
use wasmer::Value;

#[derive(Debug, Clone, PartialEq, Default)]
struct Directory {
  modules: Vec<Module>,
}

impl Directory {
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
}

impl Module {
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
  let module_wasm =
    std::fs::read("target/wasm32-unknown-unknown/debug/complogic_gates.wasm")
      .unwrap();
  let gates_module: Module = toml::from_str(
    &std::fs::read_to_string("complogic-gates/gates.toml").unwrap(),
  )
  .unwrap();

  let directory = Directory {
    modules: vec![gates_module],
  };

  let gate_index = directory
    .gate_index("std", "counter")
    .expect("Gate not found in directory");

  let mut store = wasmer::Store::default();
  let module = wasmer::Module::new(&store, module_wasm)?;
  // The module doesn't import anything, so we create an empty import object.
  let import_object = wasmer::imports! {};
  let instance = wasmer::Instance::new(&mut store, &module, &import_object)?;

  let gates = instance.exports.get_function("gates")?;
  let result = gates.call(
    &mut store,
    &[Value::I32(0), Value::I32(gate_index), Value::I32(0)],
  )?;
  assert_eq!(result[0], Value::I32(1));

  let result = gates.call(
    &mut store,
    &[Value::I32(0), Value::I32(gate_index), Value::I32(0)],
  )?;
  assert_eq!(result[0], Value::I32(2));

  Ok(())
}
