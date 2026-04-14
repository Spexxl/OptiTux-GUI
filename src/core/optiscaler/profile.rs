use crate::core::gpu_detector::GpuArchitecture;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct ProfileGenerator;

impl ProfileGenerator {
    pub fn generate_ini(game_dir: &Path, arch: &GpuArchitecture, use_dlss_spoofing: bool) -> Result<()> {
        let (dx11, dx12, vk, ffx_backend) = match arch {
            GpuArchitecture::RDNA4 | GpuArchitecture::RDNA1_2_3 => ("fsr31_12", "fsr31", "fsr31_12", "0"),
            GpuArchitecture::RTX => ("dlss", "dlss", "dlss", "auto"),
            GpuArchitecture::GTX | GpuArchitecture::IntelArc => ("xess", "xess", "xess", "auto"),
            _ => ("fsr31", "fsr31", "fsr31", "1"),
        };

        let dxgi_override = if use_dlss_spoofing { "auto" } else { "false" };

        let ini_content = format!(
            r#"; OptiTux-GUI Auto Generated Configuration

            [Upscalers]
            Dx11Upscaler={dx11}
            Dx12Upscaler={dx12}
            VulkanUpscaler={vk}

            [FSR]
            UpscalerIndex={ffx_backend}

            [Spoofing]
            Dxgi={dxgi_override}
            "#
        );

        let ini_path = game_dir.join("OptiScaler.ini");
        fs::write(&ini_path, ini_content).context("Failed to write OptiScaler.ini")?;

        Ok(())
    }
}
