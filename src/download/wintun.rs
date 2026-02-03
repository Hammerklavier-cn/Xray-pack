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

/// Download wintun
pub fn download_wintun() -> PackResult<()> {
    let url = "https://www.wintun.net/builds/wintun-0.14.1.zip";
    download_file(url, TEMP_DIR.join("wintun.zip"))?;
    log::info!("Downloaded wintun");
    Ok(())
}

/// Extract .dll according to platform. Also copy the LICENSE file.
pub fn extract_wintun(platform: WinPlatform) -> PackResult<()> {
    let zip_path = TEMP_DIR.join("wintun.zip");
    let extract_path = TEMP_DIR.join("wintun.dll");

    // 1. extract dll
    log::debug!("Extracting wintun.dll from {}...", zip_path.display());
    // create reader
    let reader =
        std::fs::File::open(&zip_path).map_err(|_| PackError::ReadFailed(zip_path.clone()))?;
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut zip_file = zip.by_path(format!("wintun/bin/{}/wintun.dll", platform))?;

    // create writer
    let mut writer = std::fs::File::create(&extract_path)
        .map_err(|_| PackError::CreateFailed(extract_path.clone()))?;

    // extract and copy
    std::io::copy(&mut zip_file, &mut writer)
        .map_err(|_| PackError::CopyFailed(zip_path.clone(), extract_path))?;

    // 2. extract license
    log::debug!("Extracting wintun LICENSE...");
    let extract_path = TEMP_DIR.join("LICENSE-wintun.txt");

    let reader =
        std::fs::File::open(&zip_path).map_err(|_| PackError::ReadFailed(zip_path.clone()))?;
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut zip_file = zip.by_path("wintun/LICENSE.txt")?;

    let mut writer = std::fs::File::create(&extract_path)
        .map_err(|_| PackError::CreateFailed(extract_path.clone()))?;

    std::io::copy(&mut zip_file, &mut writer)
        .map_err(|_| PackError::CopyFailed(zip_path.clone(), extract_path.clone()))?;

    log::info!("Extracted wintun dll and LICENSE");

    // Add wintun.dll and LICENSE-wintun.txt to collected files
    COLLECTED_FILES
        .lock()
        .unwrap()
        .push(TEMP_DIR.join("wintun.dll"));
    COLLECTED_FILES.lock().unwrap().push(extract_path);

    Ok(())
}
