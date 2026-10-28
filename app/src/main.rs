//! Hypergamy Meter: a joke desktop app for the `is_hypergamy_real` side project.
//!
//! Enter a few facts, press the button, receive a completely unscientific
//! "market verdict". For fun only. Not science, not advice.

mod gauge;
mod memes;
mod model;
mod ui;

/// Starts the native window.
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Hypergamy Meter")
            .with_inner_size([760.0, 720.0])
            .with_min_inner_size([520.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hypergamy Meter",
        options,
        Box::new(|_cc| Ok(Box::new(ui::MeterApp::default()))),
    )
}
