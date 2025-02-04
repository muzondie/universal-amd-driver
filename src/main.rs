mod app;
mod error;
mod gui;

use crate::app::AmdDriverApp;
use crate::error::AmdDriverError;

fn main() -> Result<(), AmdDriverError> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Universal AMD Driver",
        native_options,
        Box::new(|cc| Box::new(AmdDriverApp::new(cc))),
    )
}