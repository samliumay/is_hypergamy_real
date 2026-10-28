//! Hypergamy Meter: a joke desktop app for the `is_hypergamy_real` side project.
//!
//! Enter a few facts, press the button, receive a completely unscientific
//! "market verdict". For fun only. Not science, not advice.
//!
//! `cargo run --release -- --demo NAME` opens the app already scored with one
//! of the profiles in [`demo::DEMOS`] (`top`, `niche`, `its-over`,
//! `single-mother-arc`).

mod demo;
mod gauge;
mod memes;
mod model;
mod ui;

/// Starts the native window, optionally with a `--demo NAME` profile.
fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    let app = match args.iter().position(|a| a == "--demo") {
        None => ui::MeterApp::default(),
        Some(i) => match args.get(i + 1).and_then(|name| demo::find(name)) {
            Some(profile) => ui::MeterApp::scored(profile),
            None => {
                let names: Vec<&str> = demo::DEMOS.iter().map(|(n, _)| *n).collect();
                eprintln!("usage: --demo <{}>", names.join("|"));
                std::process::exit(2);
            }
        },
    };
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
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
