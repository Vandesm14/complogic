#[no_mangle]
pub extern "C" fn add_one(number: i32) -> i32 {
  number + 1
}
