use std::fs;
use std::path::Path;
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
        let mut gpus = Self::detect_via_lspci();

        if gpus.is_empty() {
            gpus = Self::detect_via_sysfs();
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

    fn detect_via_lspci() -> Vec<GpuInfo> {
        let mut gpus = Vec::new();
        if let Ok(output) = Command::new("lspci").output() {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                for line in output_str.lines() {
                    if line.contains("VGA compatible controller") || line.contains("3D controller") {
                        if let Some(gpu) = Self::parse_gpu_string(line, line.split_whitespace().next().unwrap_or("")) {
                            gpus.push(gpu);
                        }
                    }
                }
            }
        }
        gpus
    }

    fn detect_via_sysfs() -> Vec<GpuInfo> {
        let mut gpus = Vec::new();
        let drm_path = Path::new("/sys/class/drm");

        if drm_path.exists() {
            if let Ok(entries) = fs::read_dir(drm_path) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("card") && !name.contains('-') {
                        let device_path = entry.path().join("device");
                        if let Ok(class_id) = fs::read_to_string(device_path.join("class")) {
                            if class_id.starts_with("0x03") {
                                if let Some(gpu) = Self::parse_sysfs_device(&device_path) {
                                    gpus.push(gpu);
                                }
                            }
                        }
                    }
                }
            }
        }
        gpus
    }

    fn parse_sysfs_device(sysfs_path: &Path) -> Option<GpuInfo> {
        let vendor_hex = fs::read_to_string(sysfs_path.join("vendor")).unwrap_or_default().trim().to_lowercase();
        let device_hex = fs::read_to_string(sysfs_path.join("device")).unwrap_or_default().trim().to_lowercase();
        let pci_id = sysfs_path.file_name().unwrap_or_default().to_string_lossy().to_string();

        let vendor_str = match vendor_hex.as_str() {
            "0x10de" => "NVIDIA",
            "0x1002" => "AMD",
            "0x8086" => "Intel",
            _ => "Unknown",
        };

        let mut name = format!("{} GPU (Device {})", vendor_str, device_hex);
        let pci_db_paths = ["/usr/share/hwdata/pci.ids", "/usr/share/misc/pci.ids", "/var/lib/pciutils/pci.ids"];

        for db_path in pci_db_paths {
            if let Ok(content) = fs::read_to_string(db_path) {
                let vendor_id_clean = vendor_hex.replace("0x", "");
                let device_id_clean = device_hex.replace("0x", "");
                let mut in_vendor = false;

                for line in content.lines() {
                    if line.starts_with(&vendor_id_clean) {
                        in_vendor = true;
                    } else if in_vendor && line.starts_with('\t') && !line.starts_with("\t\t") {
                        let trimmed = line.trim();
                        if trimmed.starts_with(&device_id_clean) {
                            name = trimmed.replace(&device_id_clean, "").trim().to_string();
                            break;
                        }
                    } else if !line.starts_with('\t') && !line.starts_with('#') && !line.is_empty() {
                        in_vendor = false;
                    }
                }
            }
            if !name.contains("Device") { break; }
        }

        Self::parse_gpu_string(&name, &pci_id)
    }

    fn parse_gpu_string(raw_name: &str, pci_id: &str) -> Option<GpuInfo> {
        let mut name = raw_name.split("VGA compatible controller:").nth(1)
            .or_else(|| raw_name.split("3D controller:").nth(1))
            .unwrap_or(raw_name)
            .trim()
            .to_string();

        if let Some(bracket_content) = Self::extract_best_name(&name) {
            name = bracket_content;
        }

        name = name.split("(rev").next().unwrap_or(&name).trim().to_string();
        name = name.replace("Advanced Micro Devices, Inc. [AMD/ATI]", "")
                   .replace("NVIDIA Corporation", "")
                   .replace("Intel Corporation", "")
                   .trim().to_string();

        let upper_line = name.to_uppercase();
        let vendor = if upper_line.contains("NVIDIA") || upper_line.contains("GEFORCE") {
            "NVIDIA".to_string()
        } else if upper_line.contains("AMD") || upper_line.contains("ATI") || upper_line.contains("RADEON") {
            "AMD".to_string()
        } else if upper_line.contains("INTEL") || upper_line.contains("ARC") {
            "Intel".to_string()
        } else {
            "Unknown".to_string()
        };

        let arch = Self::detect_architecture(&name, &vendor);

        Some(GpuInfo {
            name,
            pci_id: pci_id.to_string(),
            vendor,
            architecture: arch,
            is_primary: false,
        })
    }

    fn extract_best_name(text: &str) -> Option<String> {
        let best_name = None;
        let mut last_bracket: Option<String> = None;
        
        let mut current_bracket = String::new();
        let mut inside = false;
        
        for c in text.chars() {
            if c == '[' {
                inside = true;
                current_bracket.clear();
            } else if c == ']' && inside {
                inside = false;
                let found = current_bracket.trim().to_string();
                let upper = found.to_uppercase();
                
                if upper.contains("RADEON") || upper.contains("GEFORCE") || upper.contains("RTX") || upper.contains("GTX") || upper.contains("ARC") {
                    return Some(found);
                }
                last_bracket = Some(found);
            } else if inside {
                current_bracket.push(c);
            }
        }
        
        best_name.or(last_bracket)
    }

    fn detect_architecture(name: &str, vendor: &str) -> GpuArchitecture {
        let upper_name = name.to_uppercase();

        if vendor == "NVIDIA" {
            if upper_name.contains("RTX") { return GpuArchitecture::RTX; }
            if upper_name.contains("GTX") { return GpuArchitecture::GTX; }
            return GpuArchitecture::OtherNvidia;
        }

        if vendor == "AMD" {
            if upper_name.contains("RX 9") || upper_name.contains("NAVI 4") { return GpuArchitecture::RDNA4; }
            if upper_name.contains("RX 5") || upper_name.contains("RX 6") || upper_name.contains("RX 7") || upper_name.contains("NAVI") {
                return GpuArchitecture::RDNA1_2_3;
            }
            return GpuArchitecture::OtherAMD;
        }

        if vendor == "Intel" {
            if upper_name.contains("ARC") { return GpuArchitecture::IntelArc; }
            return GpuArchitecture::OtherIntel;
        }

        GpuArchitecture::Unknown
    }
}
