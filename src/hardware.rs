use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub vendor: String,
    pub device_id: String,
    pub hw_type: HardwareType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardwareType {
    Gpu,
    Cpu,
    Chipset,
}

pub fn detect_hardware() -> Vec<HardwareInfo> {
    vec![
        HardwareInfo {
            vendor: "AMD".to_string(),
            device_id: "1002".to_string(),
            hw_type: HardwareType::Gpu,
        },
        HardwareInfo {
            vendor: "AMD".to_string(),
            device_id: "1022".to_string(),
            hw_type: HardwareType::Cpu,
        },
    ]
}