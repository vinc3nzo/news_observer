#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // warning as errors in release build
#![warn(clippy::all, rust_2018_idioms)]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = news_observer::NewsObserverApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
