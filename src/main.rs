use std::sync::OnceLock;
use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;

use crate::download::geodat::download_geodat;
use crate::download::wintun::{download_wintun, extract_wintun};
use crate::errors::{PackError, PackResult};
use crate::package::package_all;

mod cli;
mod compile;
mod download;
mod errors;
mod package;
mod repo;

static TEMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let temp_dir = std::env::temp_dir().join("xray-pack-temp");
    if temp_dir.exists() {
        log::debug!(
            "Removing existing temporary directory at {}",
            temp_dir.display()
        );
        std::fs::remove_dir_all(&temp_dir).unwrap_or_else(|_| {
            panic!(
                "Failed to remove existing temporary directory at {}",
                temp_dir.display()
            )
        });
    }
    log::debug!("Creating temporary directory at {}", temp_dir.display());
    std::fs::create_dir(&temp_dir).unwrap_or_else(|_| {
        panic!(
            "Failed to create temporary directory at {}",
            temp_dir.display()
        )
    });
    temp_dir
});

static REPOSITORY_DIR: OnceLock<PathBuf> = OnceLock::new();

static ARGS: OnceLock<cli::Args> = OnceLock::new();

// check prerequisites
fn check_prerequisites() -> PackResult<()> {
    // Currently only Go compiler is required.
    let prerequisites = vec!["go"];

    // check if prerequisites are in PATH
    for prerequisite in prerequisites {
        if which::which(prerequisite).is_err() {
            return Err(PackError::MissingDependency(prerequisite.to_string()));
        }
    }

    Ok(())
}

fn main() -> PackResult<()> {
    let args = ARGS.get_or_init(cli::Args::parse);

    match (std::env::var("RUST_LOG"), args.verbose) {
        (Err(_), true) => unsafe { std::env::set_var("RUST_LOG", "debug") },
        (Err(_), false) => unsafe { std::env::set_var("RUST_LOG", "info") },
        _ => {}
    };

    env_logger::init();

    // check prerequisites
    check_prerequisites()?;

    let commid = repo::setup_repository()?;

    // Build Xray-core
    compile::build_xray(&commid)?;

    download_geodat(args.download_options.region)?;

    if args.compile_options.goos == "windows" {
        download_wintun()?;
        extract_wintun(&args.compile_options.goarch)?;
    }

    package_all()?;

    // Clean
    // log::debug!("Cleaning temporary directory at {}", TEMP_DIR.display());
    // std::fs::remove_dir_all(TEMP_DIR.deref()).expect(&format!(
    //     "Failed to remove temporary directory at {}",
    //     TEMP_DIR.display()
    // ));
    Ok(())
}
