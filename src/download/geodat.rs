use crate::{TEMP_DIR, cli::Region, download::download_file, errors::PackResult};

/// Download geoip and geodat
pub fn download_geodat(region: Region) -> PackResult<()> {
    let url = region.url();
    // TODO: check downloaded file by sha256.
    download_file(format!("{}geoip.dat", url), TEMP_DIR.join("geoip.dat"))?;
    log::info!("Downloaded geoip.dat");
    download_file(format!("{}geosite.dat", url), &TEMP_DIR.join("geosite.dat"))?;
    log::info!("Downloaded geosite.dat");
    Ok(())
}
