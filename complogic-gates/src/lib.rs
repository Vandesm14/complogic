#[no_mangle]
pub extern "C" fn gates(id: u32, gate: u32, pins: u32) -> u32 {
  match gate {
    0 => and_gate(pins),
    // Add more gates as needed
    _ => 0, // Default case for unsupported gates
  }
}

fn and_gate(pins: u32) -> u32 {
  let a = pins & 1;
  let b = (pins >> 1) & 1;
  a & b
}
