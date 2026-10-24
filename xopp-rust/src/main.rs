mod app;
mod history;
mod io;
mod model;
mod render;
mod theme;
mod tools;

use app::XoppApp;
use eframe::egui;

// Application entry point for xopp-rust
fn main() -> eframe::Result<()> {
    // Configure window creation options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Xournal++ Modern (Rust)")
            .with_inner_size([1280.0, 850.0])
            .with_min_inner_size([640.0, 480.0])
            .with_active(true),
        ..Default::default()
    };

    // Launch modern GPU-accelerated event loop
    eframe::run_native(
        "Xournal++ Modern",
        native_options,
        Box::new(|_cc| Ok(Box::new(XoppApp::default()))),
    )
}
