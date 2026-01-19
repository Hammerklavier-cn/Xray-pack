use std::process::Command;

use crate::{
    ARGS, REPOSITORY_DIR, TEMP_DIR, cli,
    errors::{PackError, PackResult},
};

pub fn build_xray(commid: &str) -> PackResult<()> {
    log::debug!("Building Xray-core");
    let args = ARGS.get().unwrap();

    let output_name: &'static str = if args.compile_options.goos == "windows" {
        "xray.exe"
    } else {
        "xray"
    };
    let output_path = TEMP_DIR.join(output_name);

    // change the working directory to the repository directory
    std::env::set_current_dir(REPOSITORY_DIR.get().unwrap())
        .expect("Failed to change working directory");

    let mut cmd = Command::new("go");
    let mut ldflags = String::new();
    cmd.env("GOOS", &args.compile_options.goos)
        .env("GOARCH", &args.compile_options.goarch)
        .args([
            "build",
            "-o",
            output_path.to_str().unwrap(),
            "-trimpath",
            "-buildvcs=false",
            "-gcflags",
            &args.compile_options.gcflags,
            "-ldflags",
            args.compile_options.ldflags.as_deref().unwrap_or_else(|| {
                ldflags =
                    format!("-X github.com/xtls/xray-core/core.build={commid} -s -w -buildid=");
                &ldflags
            }),
            "-v",
            "./main",
        ]);

    cmd.spawn()
        .map_err(|e| PackError::BuildFailed(e.to_string()))?
        .wait()
        .map_err(|e| PackError::BuildFailed(e.to_string()))?;

    // print stderr
    let output = cmd
        .output()
        .map_err(|e| PackError::BuildFailed(e.to_string()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("Build failed: {}", stderr);
        return Err(PackError::BuildFailed(stderr.to_string()));
    }

    log::info!("Xray built at {}", output_path.display());

    // change the working directory back to the original directory
    std::env::set_current_dir(&*cli::ROOT).expect("Failed to change working directory");

    Ok(())
}
