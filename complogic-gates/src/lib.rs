#[link(wasm_import_module = "cl")]
extern "C" {
  fn debug(buf: *const u8, len: u32);
}

#[no_mangle]
extern "C" fn init() {
  let hello = "Hello from Rust!";
  unsafe { debug(hello.as_ptr(), hello.len() as u32) };
}

#[no_mangle]
extern "C" fn and(_id: u32, pins: u32) -> u32 {
  let a = pins & 1;
  let b = (pins >> 1) & 1;
  a & b
}

#[no_mangle]
extern "C" fn not(_id: u32, pins: u32) -> u32 {
  let a = pins & 1;
  !a
}
