use eframe::egui;
use crate::app::AmdDriverApp;

pub struct GuiState {
    pub hardware_list: Vec<crate::hardware::HardwareInfo>,
    pub current_drivers: Vec<String>,
    pub error_message: Option<String>,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            hardware_list: vec![],
            current_drivers: vec![],
            error_message: None,
        }
    }
}

pub fn show_main_window(ctx: &egui::Context, state: &mut GuiState, app: &mut AmdDriverApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("AMD Hardware Detection");
        
        if ui.button("Scan Hardware").clicked() {
            state.hardware_list = crate::hardware::detect_hardware();
        }
        
        ui.separator();
        
        for hw in &state.hardware_list {
            ui.label(format!("{}: {}", hw.hw_type, hw.device_id));
        }
        
        ui.separator();
        
        if ui.button("Install Drivers").clicked() {
            if let Err(e) = app.install_drivers() {
                state.error_message = Some(e.to_string());
            }
        }
        
        if let Some(err) = &state.error_message {
            ui.colored_label(egui::Color32::RED, err);
        }
    });
}