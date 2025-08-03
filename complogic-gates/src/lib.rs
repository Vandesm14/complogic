#[no_mangle]
extern "C" fn gates(_id: u32, gate: u32, pins: u32) -> u32 {
  match gate {
    0 => and_gate(pins),
    1 => not_gate(pins),
    _ => 0, // Default case for unsupported gates
  }
}

pub fn and_gate(pins: u32) -> u32 {
  let a = pins & 1;
  let b = (pins >> 1) & 1;
  a & b
}

pub fn not_gate(pins: u32) -> u32 {
  let a = pins & 1;
  !a
}
