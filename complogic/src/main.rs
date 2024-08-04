use wasmer::{imports, Instance, Module, Store, Value};

fn main() -> anyhow::Result<()> {
  let module_wasm =
    include_bytes!("../../complogic-gates/pkg/complogic_gates_bg.wasm");

  let mut store = Store::default();
  let module = Module::new(&store, module_wasm)?;
  // The module doesn't import anything, so we create an empty import object.
  let import_object = imports! {};
  let instance = Instance::new(&mut store, &module, &import_object)?;

  let add_one = instance.exports.get_function("add_one")?;
  let result = add_one.call(&mut store, &[Value::I32(42)])?;
  assert_eq!(result[0], Value::I32(43));

  Ok(())
}
