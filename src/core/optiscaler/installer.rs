use crate::core::models::Game;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context, Result};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjectionMethod {
    Dxgi,
    Winmm,
    Version,
    Dbghelp,
    D3d12,
    Wininet,
    Winhttp,}

impl InjectionMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dxgi => "dxgi.dll",
            Self::Winmm => "winmm.dll",
            Self::Version => "version.dll",
            Self::Dbghelp => "dbghelp.dll",
            Self::D3d12 => "d3d12.dll",
            Self::Wininet => "wininet.dll",
            Self::Winhttp => "winhttp.dll",
        }
    }

    pub fn all() -> Vec<InjectionMethod> {
        vec![
            Self::Dxgi,
            Self::Winmm,
            Self::Version,
            Self::Dbghelp,
            Self::D3d12,
            Self::Wininet,
            Self::Winhttp,
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallManifest {
    pub version: String,
    pub injection_dll: String,
}

pub struct Installer;

impl Installer {
    pub fn install(game: &Game, version_dir: &Path, version_name: &str, injection: InjectionMethod) -> Result<()> {
        let game_dir = Self::get_target_dir(game)?;
        let injection_dll = injection.as_str();

        let backup_dir = game_dir.join("OptiScalerBackup");
        let injection_path = game_dir.join(injection_dll);

        if injection_path.exists() {
            if !backup_dir.exists() {
                fs::create_dir_all(&backup_dir).context("Failed to create OptiScalerBackup directory")?;
            }
            let backup_file = backup_dir.join(injection_dll);
            if !backup_file.exists() {
                fs::copy(&injection_path, &backup_file).context("Failed to backup the original injection DLL")?;
            }
        }

        for entry in WalkDir::new(version_dir).into_iter().filter_map(|e| e.ok()) {
            let relative_path = entry.path().strip_prefix(version_dir)?;
            let target_path = game_dir.join(relative_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(&target_path).ok();
            } else if entry.file_type().is_file() {
                let file_name = entry.file_name().to_string_lossy().to_lowercase();
                
                if file_name == "optiscaler.dll" {
                    fs::copy(entry.path(), &injection_path).context("Failed to copy OptiScaler as injection DLL")?;
                } else if file_name != "setup_linux.sh" && file_name != "setup_windows.bat" {
                    fs::copy(entry.path(), &target_path).ok();
                }
            }
        }

        let manifest = InstallManifest {
            version: version_name.to_string(),
            injection_dll: injection_dll.to_string(),
        };

        let manifest_path = game_dir.join("optiscaler_manifest.json");
        let json = serde_json::to_string_pretty(&manifest)?;
        fs::write(manifest_path, json).context("Failed to write installation manifest")?;

        Ok(())
    }

    pub fn install_int8(game: &Game, int8_file_path: &Path) -> Result<()> {
        let game_dir = Self::get_target_dir(game)?;
        let target_path = game_dir.join("amd_fidelityfx_upscaler_dx12.dll");
        
        fs::copy(int8_file_path, &target_path).context("Failed to overlay INT8 DLL into Game directory")?;
        Ok(())
    }

    pub fn uninstall(game: &Game) -> Result<()> {
        let game_dir = Self::get_target_dir(game)?;
        let manifest_path = game_dir.join("optiscaler_manifest.json");

        if !manifest_path.exists() {
            return Ok(());
        }

        let json = fs::read_to_string(&manifest_path)?;
        let manifest: InstallManifest = serde_json::from_str(&json)?;

        let injection_path = game_dir.join(&manifest.injection_dll);
        if injection_path.exists() {
            let _ = fs::remove_file(&injection_path);
        }

        let backup_file = game_dir.join("OptiScalerBackup").join(&manifest.injection_dll);
        if backup_file.exists() {
            let _ = fs::copy(&backup_file, &injection_path);
            let _ = fs::remove_file(&backup_file);
        }

        let extra_files = [
            "OptiScaler.ini", "OptiScaler.log", "fakenvapi.dll", "fakenvapi.ini", 
            "fakenvapi.log", "dlssg_to_fsr3_amd_is_better.dll", "dlssg_to_fsr3.log"
        ];
        for f in extra_files {
            if let Ok(p) = game_dir.join(f).canonicalize() {
                let _ = fs::remove_file(p);
            } else {
                let _ = fs::remove_file(game_dir.join(f));
            }
        }

        let extra_dirs = ["D3D12_Optiscaler", "DlssOverrides", "Licenses"];
        for d in extra_dirs {
            let _ = fs::remove_dir_all(game_dir.join(d));
        }

        let _ = fs::remove_file(manifest_path);

        Ok(())
    }

    pub fn is_installed(game: &Game) -> bool {
        if let Ok(dir) = Self::get_target_dir(game) {
            return dir.join("optiscaler_manifest.json").exists();
        }
        false
    }

    pub fn get_installed_version(game: &Game) -> Option<String> {
        if let Ok(dir) = Self::get_target_dir(game) {
            let manifest_path = dir.join("optiscaler_manifest.json");
            if let Ok(json) = fs::read_to_string(manifest_path) {
                if let Ok(manifest) = serde_json::from_str::<InstallManifest>(&json) {
                    return Some(manifest.version);
                }
            }
        }
        None
    }

    fn get_target_dir(game: &Game) -> Result<PathBuf> {
        if let Some(exe_path) = &game.executable_path {
            if let Some(parent) = Path::new(exe_path).parent() {
                return Ok(parent.to_path_buf());
            }
        }
        Ok(PathBuf::from(&game.install_path))
    }

    fn find_optiscaler_dll(version_dir: &Path) -> Result<PathBuf> {
        for entry in WalkDir::new(version_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name == "optiscaler.dll" || name == "nvngx.dll" {
                    return Ok(entry.path().to_path_buf());
                }
            }
        }
        Err(anyhow!("OptiScaler DLL not found within the selected version directory."))
    }
}
