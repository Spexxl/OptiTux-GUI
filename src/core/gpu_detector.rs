use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GpuArchitecture {
    RDNA4,
    RDNA1_2_3,
    RTX,
    GTX,
    IntelArc,
    OtherAMD,
    OtherNvidia,
    OtherIntel,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub pci_id: String,
    pub vendor: String,
    pub architecture: GpuArchitecture,
    pub is_primary: bool,
}

pub struct GpuDetector;

impl GpuDetector {
    pub fn detect_gpus() -> Vec<GpuInfo> {
        let mut gpus = Vec::new();

        let output = Command::new("lspci")
            .output()
            .unwrap_or_else(|_| std::process::Command::new("true").output().unwrap());

        if let Ok(output_str) = String::from_utf8(output.stdout) {
            for line in output_str.lines() {
                if line.contains("VGA compatible controller") || line.contains("3D controller") {
                    if let Some(gpu) = Self::parse_lspci_line(line) {
                        gpus.push(gpu);
                    }
                }
            }
        }

        if gpus.len() > 1 {
            let mut primary_assigned = false;
            for gpu in gpus.iter_mut() {
                if !primary_assigned && (
                    gpu.architecture == GpuArchitecture::RTX ||
                    gpu.architecture == GpuArchitecture::GTX ||
                    gpu.architecture == GpuArchitecture::RDNA4 ||
                    gpu.architecture == GpuArchitecture::RDNA1_2_3 ||
                    gpu.architecture == GpuArchitecture::IntelArc
                ) {
                    gpu.is_primary = true;
                    primary_assigned = true;
                } else {
                    gpu.is_primary = false;
                }
            }
            if !primary_assigned {
                gpus[0].is_primary = true;
            }
        } else if !gpus.is_empty() {
            gpus[0].is_primary = true;
        }

        gpus
    }

    fn parse_lspci_line(line: &str) -> Option<GpuInfo> {
        let pci_id = line.split_whitespace().next()?.to_string();
        let upper_line = line.to_uppercase();
        
        let vendor = if upper_line.contains("NVIDIA") {
            "NVIDIA".to_string()
        } else if upper_line.contains("AMD") || upper_line.contains("ATI") {
            "AMD".to_string()
        } else if upper_line.contains("INTEL") {
            "Intel".to_string()
        } else {
            "Unknown".to_string()
        };

        let name = line.split("VGA compatible controller:").nth(1)
            .or_else(|| line.split("3D controller:").nth(1))
            .unwrap_or(line)
            .trim()
            .to_string();

        let arch = Self::detect_architecture(&name, &vendor);

        Some(GpuInfo {
            name,
            pci_id,
            vendor,
            architecture: arch,
            is_primary: false,
        })
    }

    fn detect_architecture(name: &str, vendor: &str) -> GpuArchitecture {
        let upper_name = name.to_uppercase();

        if vendor == "NVIDIA" {
            if upper_name.contains("RTX") {
                return GpuArchitecture::RTX;
            } else if upper_name.contains("GTX") {
                return GpuArchitecture::GTX;
            }
            return GpuArchitecture::OtherNvidia;
        }

        if vendor == "AMD" {
            if upper_name.contains("RX 9") || upper_name.contains("NAVI 4") {
                return GpuArchitecture::RDNA4;
            }
            if upper_name.contains("RX 5") || upper_name.contains("RX 6") || upper_name.contains("RX 7") || upper_name.contains("NAVI") {
                return GpuArchitecture::RDNA1_2_3;
            }
            return GpuArchitecture::OtherAMD;
        }

        if vendor == "Intel" {
            if upper_name.contains("ARC") {
                return GpuArchitecture::IntelArc;
            }
            return GpuArchitecture::OtherIntel;
        }

        GpuArchitecture::Unknown
    }
}
