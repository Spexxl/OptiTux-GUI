use crate::core::gpu_detector::GpuArchitecture;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct ProfileGenerator;

impl ProfileGenerator {
    pub fn update_ini(game_dir: &Path, arch: &GpuArchitecture, use_dlss_spoofing: bool) -> Result<()> {
        let (dx11, dx12, vk, ffx_backend) = match arch {
            GpuArchitecture::RDNA4 | GpuArchitecture::RDNA1_2_3 => ("fsr31_12", "fsr31", "fsr31_12", "0"),
            GpuArchitecture::RTX => ("dlss", "dlss", "dlss", "auto"),
            GpuArchitecture::GTX | GpuArchitecture::IntelArc => ("xess", "xess", "xess", "auto"),
            _ => ("fsr31", "fsr31", "fsr31", "1"),
        };

        let dxgi_override = if use_dlss_spoofing { "auto" } else { "false" };

        Self::patch_ini(game_dir, dx11, dx12, vk, ffx_backend, dxgi_override, None)
    }

    pub fn update_ini_custom(
        game_dir: &Path,
        upscaler: &str,
        use_dlss_spoofing: bool,
        enable_framegen: Option<bool>,
    ) -> Result<()> {
        let (dx11, dx12, vk, ffx_backend) = match upscaler {
            "fsr" => ("fsr31_12", "fsr31", "fsr31_12", "0"),
            "dlss" => ("dlss", "dlss", "dlss", "auto"),
            "xess" => ("xess", "xess", "xess", "auto"),
            _ => ("fsr31", "fsr31", "fsr31", "1"),
        };

        let dxgi_override = if use_dlss_spoofing { "auto" } else { "false" };

        Self::patch_ini(game_dir, dx11, dx12, vk, ffx_backend, dxgi_override, enable_framegen)
    }

    fn patch_ini(
        game_dir: &Path,
        dx11: &str,
        dx12: &str,
        vk: &str,
        ffx_backend: &str,
        dxgi_override: &str,
        enable_framegen: Option<bool>,
    ) -> Result<()> {
        let ini_path = game_dir.join("OptiScaler.ini");
        if !ini_path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&ini_path).context("Failed to read existing OptiScaler.ini")?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let mut current_section = String::new();

        for line in lines.iter_mut() {
            let trimmed = line.trim();

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len()-1].to_lowercase();
                continue;
            }

            if trimmed.starts_with(';') || trimmed.is_empty() {
                continue;
            }

            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();

                match current_section.as_str() {
                    "upscalers" => {
                        match key {
                            "Dx11Upscaler" => *line = format!("Dx11Upscaler={}", dx11),
                            "Dx12Upscaler" => *line = format!("Dx12Upscaler={}", dx12),
                            "VulkanUpscaler" => *line = format!("VulkanUpscaler={}", vk),
                            _ => {}
                        }
                    }
                    "fsr" => {
                        if key == "UpscalerIndex" {
                            *line = format!("UpscalerIndex={}", ffx_backend);
                        }
                    }
                    "spoofing" => {
                        if key == "Dxgi" {
                            *line = format!("Dxgi={}", dxgi_override);
                        }
                    }
                    "framegen" => {
                        if let Some(true) = enable_framegen {
                            match key {
                                "Enabled" => *line = "Enabled=true".to_string(),
                                "FGInput" => *line = "FGInput=upscaler".to_string(),
                                "FGOutput" => *line = "FGOutput=xefg".to_string(),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        fs::write(&ini_path, lines.join("\n")).context("Failed to save updated OptiScaler.ini")?;

        Ok(())
    }
}

