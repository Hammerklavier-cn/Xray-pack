use std::fmt::Display;

use crate::{
    COLLECTED_FILES, TEMP_DIR,
    download::download_file,
    errors::{PackError, PackResult},
};

pub enum WinPlatform {
    X86,
    Amd64,
    Arm,
    Arm64,
}
impl Display for WinPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WinPlatform::X86 => write!(f, "x86"),
            WinPlatform::Amd64 => write!(f, "amd64"),
            WinPlatform::Arm => write!(f, "arm"),
            WinPlatform::Arm64 => write!(f, "arm64"),
        }
    }
}
impl From<&str> for WinPlatform {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "x86" => WinPlatform::X86,
            "amd64" => WinPlatform::Amd64,
            "arm" => WinPlatform::Arm,
            "arm64" => WinPlatform::Arm64,
            _ => panic!("Invalid platform: {}", s),
        }
    }
}

const WINTUN_URL: &str = "https://www.wintun.net/builds/wintun-0.14.1.zip";

/// Download wintun and extract .dll according to platform. Also extract the LICENSE file.
pub fn download_and_extract_wintun(platform: WinPlatform) -> PackResult<()> {
    let zip_path = TEMP_DIR.join("wintun.zip");

    // Download wintun
    download_file(WINTUN_URL, &zip_path)?;
    log::info!("Downloaded wintun");

    // 1. Extract dll
    let dll_path = TEMP_DIR.join("wintun.dll");
    log::debug!("Extracting wintun.dll from {}...", zip_path.display());

    let reader =
        std::fs::File::open(&zip_path).map_err(|_| PackError::ReadFailed(zip_path.clone()))?;
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut zip_file = zip.by_path(format!("wintun/bin/{}/wintun.dll", platform))?;

    let mut writer =
        std::fs::File::create(&dll_path).map_err(|_| PackError::CreateFailed(dll_path.clone()))?;

    std::io::copy(&mut zip_file, &mut writer)
        .map_err(|_| PackError::CopyFailed(zip_path.clone(), dll_path.clone()))?;

    // 2. Extract license
    let license_path = TEMP_DIR.join("LICENSE-wintun.txt");
    log::debug!("Extracting wintun LICENSE...");

    let reader =
        std::fs::File::open(&zip_path).map_err(|_| PackError::ReadFailed(zip_path.clone()))?;
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut zip_file = zip.by_path("wintun/LICENSE.txt")?;

    let mut writer = std::fs::File::create(&license_path)
        .map_err(|_| PackError::CreateFailed(license_path.clone()))?;

    std::io::copy(&mut zip_file, &mut writer)
        .map_err(|_| PackError::CopyFailed(zip_path.clone(), license_path.clone()))?;

    log::info!("Extracted wintun dll and LICENSE");

    // Add wintun.dll and LICENSE-wintun.txt to collected files
    COLLECTED_FILES.lock().unwrap().push(dll_path);
    COLLECTED_FILES.lock().unwrap().push(license_path);

    Ok(())
}
