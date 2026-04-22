use crate::core::gpu_detector::{GpuArchitecture, GpuDetector};
use crate::core::models::Game;
use crate::core::optiscaler::github::GitHubClient;
use crate::core::optiscaler::manager::OptiScalerManager;
use crate::core::optiscaler::profile::ProfileGenerator;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

macro_rules! define_injection_methods {
    ($($variant:ident => $filename:expr, $slug:expr);* $(;)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum InjectionMethod {
            $($variant),*
        }

        impl InjectionMethod {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $filename),*
                }
            }

            pub fn from_str(s: &str) -> Self {
                match s {
                    $($slug => Self::$variant,)*
                    _ => Self::Dxgi,
                }
            }

            pub fn all() -> Vec<Self> {
                vec![$(Self::$variant),*]
            }

            pub fn all_pairs() -> Vec<(&'static str, &'static str)> {
                vec![$( ($slug, $filename) ),*]
            }
        }
    }
}

define_injection_methods! {
    Dxgi     => "dxgi.dll",     "dxgi";
    Winmm    => "winmm.dll",    "winmm";
    Version  => "version.dll",  "version";
    Dbghelp  => "dbghelp.dll",  "dbghelp";
    D3d12    => "d3d12.dll",    "d3d12";
    Wininet  => "wininet.dll",  "wininet";
    Winhttp  => "winhttp.dll",  "winhttp";
}

macro_rules! define_string_enum {
    ($name:ident, $($variant:ident => $slug:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $slug),*
                }
            }

            pub fn all_str() -> Vec<&'static str> {
                vec![$($slug),*]
            }
        }
    }
}

define_string_enum! {
    FGInput,
    Nukems   => "nukems",
    Dlssg    => "dlssg",
    Fsrfg    => "fsrfg",
    Upscaler => "upscaler",
    Fsrfg30  => "fsrfg30",
    Nofg     => "nofg",
}

define_string_enum! {
    FGOutput,
    Nukems => "nukems",
    Fsrfg  => "fsrfg",
    Xefg   => "xefg",
    Nofg   => "nofg",
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallManifest {
    pub version: String,
    pub injection_dll: String,
}

pub struct Installer;

impl Installer {
    pub fn install(
        game: &Game,
        version_dir: &Path,
        version_name: &str,
        injection: InjectionMethod,
    ) -> Result<()> {
        let game_dir = Self::get_target_dir(game)?;
        let injection_dll = injection.as_str();

        let backup_dir = game_dir.join("OptiScalerBackup");
        let injection_path = game_dir.join(injection_dll);

        if injection_path.exists() {
            if !backup_dir.exists() {
                fs::create_dir_all(&backup_dir)
                    .context("Failed to create OptiScalerBackup directory")?;
            }
            let backup_file = backup_dir.join(injection_dll);
            if !backup_file.exists() {
                fs::copy(&injection_path, &backup_file)
                    .context("Failed to backup the original injection DLL")?;
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
                    fs::copy(entry.path(), &injection_path)
                        .context("Failed to copy OptiScaler as injection DLL")?;
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

        fs::copy(int8_file_path, &target_path)
            .context("Failed to overlay INT8 DLL into Game directory")?;
        Ok(())
    }

    pub fn uninstall(game: &Game) -> Result<()> {
        let game_dir = Self::get_target_dir(game)?;
        let manifest_path = game_dir.join("optiscaler_manifest.json");

        let mut removed_any = false;

        if manifest_path.exists() {
            let json = fs::read_to_string(&manifest_path)
                .context("Failed to read optiscaler_manifest.json")?;
            let manifest: InstallManifest =
                serde_json::from_str(&json).context("Failed to parse optiscaler_manifest.json")?;

            let dll_path = game_dir.join(&manifest.injection_dll);
            if dll_path.exists() {
                let backup_file = game_dir
                    .join("OptiScalerBackup")
                    .join(&manifest.injection_dll);
                if backup_file.exists() {
                    if fs::copy(&backup_file, &dll_path).is_ok() {
                        let _ = fs::remove_file(&backup_file);
                        removed_any = true;
                    }
                } else if fs::remove_file(&dll_path).is_ok() {
                    removed_any = true;
                }
            }

            if fs::remove_file(&manifest_path).is_ok() {
                removed_any = true;
            }
        }

        let extra_files = [
            "OptiScaler.ini",
            "OptiScaler.log",
            "fakenvapi.dll",
            "fakenvapi.ini",
            "fakenvapi.log",
            "dlssg_to_fsr3_amd_is_better.dll",
            "dlssg_to_fsr3.log",
        ];

        if let Ok(entries) = fs::read_dir(&game_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    let lower_name = file_name.to_lowercase();
                    if extra_files.iter().any(|&f| f.to_lowercase() == lower_name) {
                        let _ = fs::remove_file(entry.path());
                        removed_any = true;
                    }
                }
            }
        }

        let extra_dirs = [
            "D3D12_Optiscaler",
            "DlssOverrides",
            "Licenses",
            "OptiScalerBackup",
        ];
        for d in extra_dirs {
            let dir_path = game_dir.join(d);
            if dir_path.exists() {
                let _ = fs::remove_dir_all(&dir_path);
            }
        }

        if !removed_any {
            return Err(anyhow!(
                "No OptiScaler files were found to remove in {:?}",
                game_dir
            ));
        }

        Ok(())
    }

    pub fn is_installed(game: &Game) -> bool {
        let Ok(dir) = Self::get_target_dir(game) else {
            return false;
        };

        if dir.join("optiscaler_manifest.json").exists() {
            return true;
        }

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    let lower_name = file_name.to_lowercase();
                    if lower_name == "optiscaler.ini" {
                        return true;
                    }
                }
            }
        }

        let backup_dir = dir.join("OptiScalerBackup");
        if backup_dir.exists() {
            if let Ok(entries) = fs::read_dir(&backup_dir) {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if dir.join(&file_name).exists() {
                            return true;
                        }
                    }
                }
            }
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

    pub async fn quick_install<F>(game: &Game, on_progress: F) -> Result<()>
    where
        F: Fn(&str, f64),
    {
        on_progress("starting", 0.0);

        let gpu = GpuDetector::detect_gpus()
            .into_iter()
            .find(|g| g.is_primary);
        let arch = gpu
            .map(|g| g.architecture)
            .unwrap_or(GpuArchitecture::Unknown);

        let needs_int8 = matches!(arch, GpuArchitecture::RDNA1_2_3);
        let injection = InjectionMethod::Dxgi;

        let downloaded_versions = OptiScalerManager::get_downloaded_versions();
        let stable_folder = downloaded_versions
            .into_iter()
            .find(|v| !v.to_lowercase().contains("db"));

        let version_path = if let Some(folder_name) = stable_folder {
            on_progress("installing", 20.0);
            OptiScalerManager::get_version_path(&folder_name)
                .ok_or_else(|| anyhow!("Could not resolve local version path."))?
        } else {
            on_progress("fetching", 10.0);

            let releases = GitHubClient::get_latest_releases().await?;

            let latest_stable = releases
                .into_iter()
                .find(|r| r.source == "stable" && !r.prerelease)
                .ok_or_else(|| anyhow!("No stable release found online."))?;

            let asset = latest_stable
                .assets
                .into_iter()
                .find(|a| a.name.ends_with(".zip") || a.name.ends_with(".7z"))
                .ok_or_else(|| anyhow!("No downloadable asset found in latest stable release."))?;

            on_progress("downloading", 20.0);

            let extract_dir = OptiScalerManager::download_and_extract(&asset).await?;
            on_progress("installing", 80.0);
            extract_dir
        };

        if needs_int8 && !OptiScalerManager::is_int8_present() {
            if let Ok(int8_asset) = GitHubClient::get_int8_addon().await {
                on_progress("downloading_int8", 60.0);
                let _ = OptiScalerManager::download_int8(&int8_asset).await;
            }
        }

        let version_name = version_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Self::install(game, &version_path, &version_name, injection)?;

        let use_dlss_spoofing = matches!(
            arch,
            GpuArchitecture::RDNA4
                | GpuArchitecture::RDNA1_2_3
                | GpuArchitecture::IntelArc
                | GpuArchitecture::GTX
        );

        let game_dir = Self::get_target_dir(game)?;
        let _ = ProfileGenerator::update_ini(&game_dir, &arch, use_dlss_spoofing);

        if needs_int8 {
            if let Some(int8_path) = OptiScalerManager::int8_path_pub() {
                if int8_path.exists() {
                    let _ = Self::install_int8(game, &int8_path);
                }
            }
        }

        on_progress("done", 100.0);

        Ok(())
    }

    pub async fn custom_install<F>(
        game: &Game,
        version_folder: &str,
        upscaler: &str,
        install_int8: bool,
        enable_framegen: bool,
        is_mfg: bool,
        injection: InjectionMethod,
        fg_input: &str,
        fg_output: &str,
        on_progress: F,
    ) -> Result<()>
    where
        F: Fn(&str, f64),
    {
        on_progress("starting", 0.0);

        let version_path = OptiScalerManager::get_version_path(version_folder)
            .ok_or_else(|| anyhow!("Could not resolve version path for '{}'.", version_folder))?;

        on_progress("installing", 40.0);

        let version_name = version_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Self::install(game, &version_path, &version_name, injection)?;

        on_progress("configuring", 70.0);

        let use_dlss_spoofing = upscaler != "dlss";

        let game_dir = Self::get_target_dir(game)?;
        let _ = ProfileGenerator::update_ini_custom(
            &game_dir,
            upscaler,
            use_dlss_spoofing,
            if is_mfg { None } else { Some(enable_framegen) },
            fg_input,
            fg_output,
        );

        if install_int8 {
            if !OptiScalerManager::is_int8_present() {
                if let Ok(int8_asset) = GitHubClient::get_int8_addon().await {
                    on_progress("downloading_int8", 80.0);
                    let _ = OptiScalerManager::download_int8(&int8_asset).await;
                }
            }
            if let Some(int8_path) = OptiScalerManager::int8_path_pub() {
                if int8_path.exists() {
                    let _ = Self::install_int8(game, &int8_path);
                }
            }
        }

        on_progress("done", 100.0);

        Ok(())
    }

    pub(crate) fn get_target_dir(game: &Game) -> Result<PathBuf> {
        if let Some(exe_path) = &game.executable_path {
            if let Some(parent) = Path::new(exe_path).parent() {
                return Ok(parent.to_path_buf());
            }
        }
        Ok(PathBuf::from(&game.install_path))
    }
}
