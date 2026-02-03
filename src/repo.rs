use git2::{FetchOptions, ProxyOptions, Repository, build::RepoBuilder};

use crate::{ARGS, REPOSITORY_DIR, TEMP_DIR, errors::PackResult};

fn get_https_proxy() -> Option<String> {
    if let Ok(proxy) = std::env::var("HTTPS_PROXY") {
        Some(proxy)
    } else if let Ok(proxy) = std::env::var("https_proxy") {
        Some(proxy)
    } else if let Ok(proxy) = std::env::var("ALL_PROXY") {
        Some(proxy)
    } else {
        std::env::var("all_proxy").ok()
    }
}

/// This function returns `git describe` result
pub fn setup_repository() -> PackResult<String> {
    // Open or clone Xray-core repository
    let args = ARGS.get().unwrap();
    let repo: Repository = if args.path_options.from_source {
        // Clone Xray-core source code (via proxy if available)
        let dest = TEMP_DIR.join(args.target.to_string());

        match get_https_proxy() {
            Some(ref proxy) => {
                log::debug!(
                    "Cloning {} repository to {} with proxy {}",
                    args.target,
                    dest.display(),
                    proxy
                );
                let mut proxy_opts = ProxyOptions::new();
                proxy_opts.url(proxy);
                let mut fetch_opts = FetchOptions::new();
                fetch_opts.proxy_options(proxy_opts);

                RepoBuilder::new()
                    .fetch_options(fetch_opts)
                    .clone(args.target.repo_url(), &dest)?
            }
            None => {
                log::debug!("Cloning {} repository to {}", args.target, dest.display());
                RepoBuilder::new().clone(args.target.repo_url(), &dest)?
            }
        }
    } else {
        log::debug!(
            "Open {} repository at {}",
            args.target,
            &args.path_options.source_path.display()
        );
        Repository::open(&args.path_options.source_path)?
    };
    REPOSITORY_DIR
        .set(repo.path().join("../").to_path_buf())
        .unwrap();
    log::info!(
        "{} repository locates at {}",
        args.target,
        REPOSITORY_DIR.get().unwrap().display()
    );

    // Checkout Xray-core version
    log::debug!(
        "Checking out {} version {}",
        args.target,
        args.target.repo_version()
    );
    let (object, reference) = repo.revparse_ext(&args.target.repo_version())?;
    repo.checkout_tree(&object, None)?;
    if let Some(reference) = reference {
        repo.set_head(reference.name().unwrap())?;
    } else {
        repo.set_head_detached(object.id())?;
    }
    log::info!(
        "Switch to {} version {}",
        args.target,
        args.target.repo_version()
    );

    // Get result of (git describe --always --dirty)
    let describe_result = repo
        .describe(git2::DescribeOptions::new().describe_tags())
        .and_then(|describe| describe.format(None))
        .unwrap_or_else(|_| object.id().to_string());

    log::info!("Current commit id: {}", describe_result);

    Ok(describe_result)
}
