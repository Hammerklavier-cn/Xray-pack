use crate::{TEMP_DIR, download::download_file, errors::PackResult};

/// Download wintun
pub fn download_wintun() -> PackResult<()> {
    let url = "https://www.wintun.net/builds/wintun-0.14.1.zip";
    download_file(url, TEMP_DIR.join("wintun.zip"))?;
    log::info!("Downloaded wintun");
    Ok(())
}

/// Extract .dll according to platform. Also copy the LICENSE file.
pub fn extract_wintun(platform: impl AsRef<str>) -> PackResult<()> {
    let zip_path = TEMP_DIR.join("wintun.zip");
    let extract_path = TEMP_DIR.join("wintun.dll");

    // 1. extract dll
    log::debug!("Extracting wintun.dll from {}...", zip_path.display());
    // create reader
    let reader = std::fs::File::open(&zip_path)?;
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut zip_file = zip.by_path(&format!("wintun/bin/{}/wintun.dll", platform.as_ref()))?;

    // create writer
    let mut writer = std::fs::File::create(extract_path)?;

    // extract and copy
    std::io::copy(&mut zip_file, &mut writer)?;

    // 2. extract license
    log::debug!("Extracting wintun LICENSE...");
    let extract_path = TEMP_DIR.join("LICENSE-wintun.txt");

    let reader = std::fs::File::open(&zip_path)?;
    let mut zip = zip::ZipArchive::new(reader)?;
    let mut zip_file = zip.by_path("wintun/LICENSE.txt")?;

    let mut writer = std::fs::File::create(extract_path)?;

    std::io::copy(&mut zip_file, &mut writer)?;

    log::info!("Extracted wintun dll and LICENSE");
    Ok(())
}
