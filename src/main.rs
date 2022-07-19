#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::Vec2;

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // let mut native_options = eframe::NativeOptions::default();
    let winsize = Vec2::new(800., 800.);
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(winsize),
        max_window_size: Some(winsize),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "mygcalc",
        native_options,
        Box::new(|cc| Box::new(mygcalc::TemplateApp::new(cc))),
    );
}
