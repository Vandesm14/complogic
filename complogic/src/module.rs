use std::path::{Path, PathBuf};

use internment::Intern;
use serde::Deserialize;
use wasmer::{Function, FunctionEnv, FunctionEnvMut, Imports, Instance, Memory, Store};

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub struct ModuleConfig {
  pub id: Intern<String>,
  pub name: Option<String>,
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

    let mut imports = Imports::new();

    let debug_env = FunctionEnv::new(&mut store, None);

    imports.define("cl", "debug", Function::new_typed_with_env(&mut store, &debug_env, debug));

    let instance = Instance::new(&mut store, &wasm_module, &imports)
      .expect("Failed to instantiate module");

    *debug_env.as_mut(&mut store) = Some(instance.exports.get_memory("memory").unwrap().clone());

    if let Ok(init) = instance.exports.get_function("init") {
      init.call(&mut store, &[]).unwrap_or_else(|_| {
        panic!("Failed to initialize module: {}", module.module.id)
      });
    }

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
  pub id: Intern<String>,
  pub name: Option<String>,
  pub description: Option<String>,
  pub inputs: u32,
  pub outputs: u32,
}

pub type GateEnv = Option<Memory>;

fn debug(mut env: FunctionEnvMut<GateEnv>, ptr: u32, len: u32) {
  let (data, store) = env.data_and_store_mut();
  let memory = data.as_ref().unwrap().view(&store);

  let buf = memory.copy_range_to_vec(ptr as u64..ptr as u64 + len as u64).unwrap();

  unsafe {
    let slice = core::str::from_utf8_unchecked(&buf);
    eprintln!("{slice}");
  }
}
