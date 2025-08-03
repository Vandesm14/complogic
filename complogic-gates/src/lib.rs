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
