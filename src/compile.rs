use std::process::Command;

use crate::{
    ARGS, REPOSITORY_DIR, TEMP_DIR,
    cli::{self, CompileTarget},
    errors::{PackError, PackResult},
};

pub fn build_xray(commid: &str) -> PackResult<()> {
    log::debug!("Building Xray-core");
    let args = ARGS.get().unwrap();

    let output_name: String = {
        let s = match args.target {
            CompileTarget::V2ray {
                compile_options: _,
                v2ray_version: _,
            } => "v2ray",
            CompileTarget::Xray {
                compile_options: _,
                xray_version: _,
            } => "xray",
        };
        let goos = &args.go_target.goos;
        if goos == "windows" {
            format!("{s}.exe")
        } else {
            s.to_string()
        }
    };
    let output_path = TEMP_DIR.join(output_name);

    // change the working directory to the repository directory
    std::env::set_current_dir(REPOSITORY_DIR.get().unwrap())
        .expect("Failed to change working directory");

    let mut cmd = Command::new("go");
    let gcflags: String = match &args.target {
        CompileTarget::V2ray {
            compile_options,
            v2ray_version: _,
        } => compile_options.gcflags.clone(),
        CompileTarget::Xray {
            compile_options,
            xray_version: _,
        } => compile_options.gcflags.clone(),
    };
    let ldflags: String = match &args.target {
        CompileTarget::V2ray {
            compile_options,
            v2ray_version: _,
        } => compile_options
            .ldflags
            .clone()
            .unwrap_or("-s -w -buildid=".to_string()),
        CompileTarget::Xray {
            compile_options,
            xray_version: _,
        } => compile_options.ldflags.clone().unwrap_or_else(|| {
            format!("-X github.com/xtls/xray-core/core.build={commid} -s -w -buildid=")
        }),
    };

    let build_args = {
        let mut vec = vec![
            "build",
            "-o",
            output_path.to_str().unwrap(),
            "-trimpath",
            "-gcflags",
            &gcflags,
            "-ldflags",
            &ldflags,
        ];
        if args.verbose {
            vec.push("-v")
        }
        match args.target {
            CompileTarget::V2ray {
                compile_options: _,
                v2ray_version: _,
            } => {}
            CompileTarget::Xray {
                compile_options: _,
                xray_version: _,
            } => vec.push("-buildvcs=false"),
        }
        vec.push("./main");
        vec
    };

    cmd.env("GOOS", &args.go_target.goos)
        .env("GOARCH", &args.go_target.goarch)
        .env(
            "CGO_ENABLED",
            std::env::var("CGO_ENABLED").unwrap_or_else(|_| "0".to_string()),
        )
        .args(&build_args);

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
