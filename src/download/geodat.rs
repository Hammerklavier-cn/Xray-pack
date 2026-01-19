use crate::{
    TEMP_DIR,
    cli::Region,
    download::{download_file, download_file_content},
    errors::PackResult,
};
use sha2::{Digest, Sha256};
use std::fs;

/// Download geoip and geodat
pub fn download_geodat(region: Region) -> PackResult<()> {
    let url = region.url();

    // Download geoip.dat
    let geoip_path = TEMP_DIR.join("geoip.dat");
    download_file(format!("{url}geoip.dat"), &geoip_path)?;
    log::info!("Downloaded geoip.dat");

    // Get expected checksum for geoip.dat
    let geoip_checksum = download_file_content(format!("{url}geoip.dat.sha256sum"))?
        .trim()
        .to_string();

    // Verify geoip.dat checksum
    verify_sha256(
        &geoip_path,
        geoip_checksum.split_whitespace().next().unwrap(),
    )?;
    log::info!("Verified geoip.dat checksum");

    // Download geosite.dat
    let geosite_path = TEMP_DIR.join("geosite.dat");
    download_file(format!("{url}geosite.dat"), &geosite_path)?;
    log::info!("Downloaded geosite.dat");

    // Get expected checksum for geosite.dat
    let geosite_checksum = download_file_content(format!("{url}geosite.dat.sha256sum"))?
        .trim()
        .to_string();

    // Verify geosite.dat checksum
    verify_sha256(
        &geosite_path,
        geosite_checksum.split_whitespace().next().unwrap(),
    )?;
    log::info!("Verified geosite.dat checksum");

    Ok(())
}

/// Verify the SHA256 checksum of a file
fn verify_sha256(file_path: &std::path::Path, expected_checksum: &str) -> PackResult<()> {
    // Read the file contents
    let file_contents = fs::read(file_path)?;

    // Calculate the SHA256 hash
    let mut hasher = Sha256::new();
    hasher.update(&file_contents);
    let calculated_hash = hasher.finalize();

    // Convert the hash to hex string
    let calculated_checksum = format!("{:x}", calculated_hash);

    // Compare the calculated checksum with the expected one
    if calculated_checksum != expected_checksum {
        return Err(crate::errors::PackError::ChecksumFailed {
            expected: expected_checksum.to_string(),
            got: calculated_checksum,
        });
    }

    Ok(())
}
