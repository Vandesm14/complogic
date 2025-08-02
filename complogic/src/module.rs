use std::path::{Path, PathBuf};

use serde::Deserialize;
use wasmer::{Imports, Instance, Store};

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub struct ModuleConfig {
  pub id: String,
  pub name: String,
  pub src: PathBuf,
}

#[derive(Debug, PartialEq, Default, Deserialize)]
pub struct Module {
  pub module: ModuleConfig,
  pub gates: Vec<Gate>,

  #[serde(skip)]
  pub instance: Option<Instance>,
  #[serde(skip)]
  pub store: Option<Store>,
}

impl Module {
  pub fn from_file(toml_path: impl AsRef<Path>) -> Self {
    let module: Module = toml::from_str(
      &std::fs::read_to_string(toml_path).expect("Failed to read module file"),
    )
    .expect("Failed to parse toml file");

    let mut store = Store::default();
    let wasm_module =
      wasmer::Module::from_file(&store, module.module.src.clone())
        .expect("Failed to load module");
    let instance = Instance::new(&mut store, &wasm_module, &Imports::new())
      .expect("Failed to instantiate module");

    module.with_instance(instance).with_store(store)
  }

  pub fn with_instance(mut self, instance: Instance) -> Self {
    self.instance = Some(instance);
    self
  }

  pub fn with_store(mut self, store: Store) -> Self {
    self.store = Some(store);
    self
  }

  pub fn wasm(&mut self) -> (&Instance, &mut Store) {
    let instance = self.instance.as_ref().expect("Module instance not set");
    let store = self.store.as_mut().expect("Module store not set");
    (instance, store)
  }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub struct Gate {
  pub id: String,
  pub name: String,
  pub description: String,
  pub inputs: i32,
  pub outputs: i32,
}
