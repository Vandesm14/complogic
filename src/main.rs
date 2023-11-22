#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// use complogic::NodeGraphExample;

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

  use complogic::{Compiler, FullAdder, Gate};

  let mut compiler = Compiler::new(3);
  let [a, b, cin] = [0, 1, 2];

  let s = compiler.alloc();
  let cout = compiler.alloc();

  let full_adder = FullAdder { a, b, cin, s, cout };

  let mut simulation = compiler.compile(vec![&Gate::from(full_adder)]);
  simulation.run(&[false, false, false]);
  println!("{:#?}", simulation);

  println!("s: {}", simulation.registers[s]);
  println!("cout: {}", simulation.registers[cout]);
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
