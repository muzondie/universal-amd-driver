use crate::error::AmdDriverError;
use crate::gui::GuiState;

pub struct AmdDriverApp {
    state: GuiState,
    driver_manager: crate::drivers::DriverManager,
}

impl AmdDriverApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: GuiState::new(),
            driver_manager: crate::drivers::DriverManager::new().unwrap(),
        }
    }
    
    pub fn install_drivers(&mut self) -> Result<(), AmdDriverError> {
        let manifest = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(crate::http::fetch_manifest())?;
        
        for hw in &self.state.hardware_list {
            if let Some(url) = manifest["drivers"][&hw.device_id].as_str() {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(self.driver_manager.install_driver(url))?;
            }
        }
        Ok(())
    }
}

impl eframe::App for AmdDriverApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        crate::gui::show_main_window(ctx, &mut self.state, self);
    }
}