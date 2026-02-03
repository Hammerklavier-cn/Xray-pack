use std::fs;

use crate::{COLLECTED_FILES, REPOSITORY_DIR, TEMP_DIR, errors::PackResult};

/// Copy v2ray systemd service files from the repository (v2ray.service and v2ray@.service)
pub fn copy_v2ray_services() -> PackResult<()> {
    let repo_dir = REPOSITORY_DIR.get().unwrap();
    let service_dir = repo_dir.join("release/config/systemd/system");

    // Copy v2ray.service
    let src = service_dir.join("v2ray.service");
    let dest = TEMP_DIR.join("v2ray.service");
    fs::copy(&src, &dest)?;
    log::info!("Copied v2ray.service");

    // Add v2ray.service to collected files
    COLLECTED_FILES.lock().unwrap().push(dest);

    // Copy v2ray@.service
    let src = service_dir.join("v2ray@.service");
    let dest = TEMP_DIR.join("v2ray@.service");
    fs::copy(&src, &dest)?;
    log::info!("Copied v2ray@.service");

    // Add v2ray@.service to collected files
    COLLECTED_FILES.lock().unwrap().push(dest);

    Ok(())
}
