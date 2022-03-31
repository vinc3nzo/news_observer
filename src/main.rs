#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = news_observer::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
