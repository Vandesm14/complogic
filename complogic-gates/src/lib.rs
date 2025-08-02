use std::sync::Mutex;

static COUNTER: Mutex<u32> = Mutex::new(0);

#[no_mangle]
extern "C" fn gates(id: u32, gate: u32, pins: u32) -> u32 {
  match gate {
    0 => and_gate(pins),
    1 => counter(),
    _ => 0, // Default case for unsupported gates
  }
}

pub fn and_gate(pins: u32) -> u32 {
  let a = pins & 1;
  let b = (pins >> 1) & 1;
  a & b
}

pub fn counter() -> u32 {
  let mut counter = COUNTER.lock().unwrap();
  *counter += 1;
  *counter
}
