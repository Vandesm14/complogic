#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

fn number_to_bin_vec(number: usize, size: usize) -> Vec<bool> {
  let mut vec = vec![];
  let mut number = number;
  while number > 0 {
    vec.push(number % 2 == 1);
    number /= 2;
  }

  while vec.len() < size {
    vec.push(false);
  }

  vec.reverse();
  vec
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
  // use eframe::egui::Visuals;

  // eframe::run_native(
  //   "Egui node graph example",
  //   eframe::NativeOptions::default(),
  //   Box::new(|cc| {
  //     cc.egui_ctx.set_visuals(Visuals::dark());
  //     Box::new(NodeGraphExample::new(cc))
  //   }),
  // )
  // .expect("Failed to run native example");

  // use complogic::{Compiler, FullAdder, Gate};

  // let mut compiler = Compiler::new(3);
  // let [a, b, cin] = [0, 1, 2];

  // let s = compiler.alloc();
  // let cout = compiler.alloc();

  // let full_adder = FullAdder { a, b, cin, s, cout };

  // let mut simulation = compiler.compile(vec![&Gate::from(full_adder)]);
  // simulation.run(&[false, false, false]);
  // println!("{:#?}", simulation);

  // println!("s: {}", simulation.registers[s]);
  // println!("cout: {}", simulation.registers[cout]);

  use complogic::{Compiler, FourBitAdder, Gate};

  let mut compiler = Compiler::new(8);
  let [a4, a3, a2, a1, b4, b3, b2, b1] = [0, 1, 2, 3, 4, 5, 6, 7];

  compiler.incrementer.skip(5);
  let [s5, s4, s3, s2, s1] = [8, 9, 10, 11, 12];

  let four_bit_adder = FourBitAdder {
    a1,
    a2,
    a3,
    a4,
    b1,
    b2,
    b3,
    b4,
    s1,
    s2,
    s3,
    s4,
    cout: s5,
  };

  let mut simulation = compiler.compile(vec![&Gate::from(four_bit_adder)]);

  for a in 0..0b1111 {
    let bin_a = number_to_bin_vec(a, 4);

    for b in 0..0b1111 {
      let bin_b = number_to_bin_vec(b, 4);

      let mut input = vec![];
      input.extend(bin_a.clone());
      input.extend(bin_b);

      simulation.run(&input);

      let bin_s = number_to_bin_vec(a + b, 5);
      assert_eq!(bin_s, &simulation.registers[s5..=s1]);
    }
  }
}

// #[cfg(target_arch = "wasm32")]
// fn main() {
//   // Redirect `log` message to `console.log` and friends:
//   // eframe::WebLogger::init(log::LevelFilter::Debug).ok();

//   let web_options = eframe::WebOptions::default();

//   wasm_bindgen_futures::spawn_local(async {
//     eframe::web::start(
//       "the_canvas_id", // hardcode it
//       web_options,
//       Box::new(|cc| Box::new(NodeGraphExample::new(cc))),
//     )
//     .await
//     .expect("failed to start eframe");
//   });
// }
