#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
  use complogic_gui::NodeGraphExample;
  use eframe::egui::Visuals;

  eframe::run_native(
    "Egui node graph example",
    eframe::NativeOptions::default(),
    Box::new(|cc| {
      cc.egui_ctx.set_visuals(Visuals::dark());
      Box::new(NodeGraphExample::new(cc))
    }),
  )
  .expect("Failed to run native example");
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
