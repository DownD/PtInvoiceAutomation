//#![cfg_attr(not(debug_assertions)] // hide console window on Windows in release

use eframe::egui;

pub mod qr_code;
pub mod invoice;
pub mod cv_worker;
pub mod constants;
mod cv_pipeline;
mod ui;
use ui::InvoiceUI;
use invoice::invoice_manager::InvoiceManager;
use std::sync::mpsc;
use env_logger;
use cv_worker::CVWorker;
use constants::SourceType;
use log::info;


fn run_pipeline_thread(tx_img : mpsc::Sender<Box<egui::ColorImage>>, tx_qr : mpsc::Sender<Box<qr_code::QRCode>>, rx_focus : mpsc::Receiver<u8>, rx_source : mpsc::Receiver<SourceType>){
    let mut cv_worker = CVWorker::create_pipeline(tx_img, tx_qr, rx_focus, rx_source);
    cv_worker.run();
    info!("Pipeline thread exited");
}

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //    tracing_subscriber::fmt::init();
    //https://users.rust-lang.org/t/best-way-to-define-a-callback-closure-that-captures-its-environment/74877/2

    env_logger::init();

    let (tx_img, rx_img) = mpsc::channel::<Box<egui::ColorImage>>();
    let (tx_qr, rx_qr) = mpsc::channel::<Box<qr_code::QRCode>>();
    let (tx_focus, rx_focus) = mpsc::channel::<u8>();
    let (tx_source, rx_source) = mpsc::channel::<SourceType>();

    std::thread::spawn(move || {
        run_pipeline_thread(tx_img,tx_qr, rx_focus, rx_source);
    });

    let invoice_manager = InvoiceManager::new(rx_qr);

    let mut invoice_ui = Box::new(InvoiceUI::new(invoice_manager));
    invoice_ui.set_thread_reciever(rx_img);
    invoice_ui.set_focus_sender(tx_focus);
    invoice_ui.set_source_sender(tx_source);

    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(768.0, 640.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Invoice Parser",
        options,
        Box::new(|_cc| invoice_ui),
    )
}
