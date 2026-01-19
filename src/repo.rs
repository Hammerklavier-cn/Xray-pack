use git2::Repository;

use crate::{ARGS, REPOSITORY_DIR, TEMP_DIR, errors::PackResult};

// Return `go describe` result
pub fn setup_repository() -> PackResult<String> {
    // Open or clone Xray-core repository
    let args = ARGS.get().unwrap();
    let repo: Repository = if args.path_options.from_source {
        // Download Xray-core source code
        let dest = TEMP_DIR.join("Xray-core");
        log::debug!("Downloading Xray-core repository to {}", dest.display());
        Repository::clone("https://github.com/XTLS/Xray-core.git", &dest)?
    } else {
        log::debug!(
            "Open Xray-core-repository at {}",
            &args.path_options.source_path.display()
        );
        Repository::open(&args.path_options.source_path)?
    };
    REPOSITORY_DIR
        .set(repo.path().join("../").to_path_buf())
        .unwrap();
    log::info!(
        "Xray-core repository locates at {}",
        REPOSITORY_DIR.get().unwrap().display()
    );

    // Checkout Xray-core version
    log::debug!("Checking out Xray-core version {}", args.xray_version);
    let (object, reference) = repo.revparse_ext(&args.xray_version)?;
    repo.checkout_tree(&object, None)?;
    if let Some(reference) = reference {
        repo.set_head(reference.name().unwrap())?;
    } else {
        repo.set_head_detached(object.id())?;
    }
    log::info!("Switch to Xray-core version {}", args.xray_version);

    // Get result of (git describe --always --dirty)
    let describe_result = repo
        .describe(git2::DescribeOptions::new().describe_tags())
        .and_then(|describe| describe.format(None))
        .unwrap_or_else(|_| object.id().to_string());

    log::info!("Current commit id: {}", describe_result);

    Ok(describe_result)
}
