//#![cfg_attr(not(debug_assertions)] // hide console window on Windows in release

use eframe::egui;

pub mod qr_code;
pub mod invoice;
mod ui;
use ui::InvoiceUI;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(InvoiceUI::default())),
    )
}
