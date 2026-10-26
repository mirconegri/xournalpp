use xopp_rust::XoppApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Xournal++ Modern (Rust)")
            .with_inner_size([1280.0, 850.0])
            .with_min_inner_size([640.0, 480.0])
            .with_active(true),
        ..Default::default()
    };

    eframe::run_native(
        "Xournal++ Modern",
        native_options,
        Box::new(|_cc| Ok(Box::new(XoppApp::default()))),
    )
}
