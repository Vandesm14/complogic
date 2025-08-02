use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> anyhow::Result<()> {
  let module_wasm =
    std::fs::read("target/wasm32-unknown-unknown/debug/complogic_gates.wasm")
      .unwrap();

  let mut store = Store::default();
  let module = Module::new(&store, module_wasm)?;
  // The module doesn't import anything, so we create an empty import object.
  let import_object = imports! {};
  let instance = Instance::new(&mut store, &module, &import_object)?;

  let gates = instance.exports.get_function("gates")?;
  let result = gates.call(
    &mut store,
    &[Value::I32(0), Value::I32(0), Value::I32(0b11)],
  )?;
  assert_eq!(result[0], Value::I32(1));

  Ok(())
}
