use std::path::PathBuf;

use crate::{
    ARGS, COLLECTED_FILES, REPOSITORY_DIR, TEMP_DIR, cli::CompileTarget, errors::PackResult,
};

/// Copy all necessary files to a directory. The path of this directory is returned.
#[deprecated(note = "Use compress_zip instead.")]
#[allow(dead_code)]
fn copy_to_dir() -> PackResult<PathBuf> {
    let args = ARGS.get().unwrap();
    let repo_dir = REPOSITORY_DIR.get().unwrap();

    let dir_name = match &args.target {
        CompileTarget::V2ray {
            compile_options: _,
            v2ray_version,
        } => {
            format! {
                "v2ray-{}-{}-{}",
                v2ray_version, args.go_target.goarch, args.go_target.goos,
            }
        }
        CompileTarget::Xray {
            compile_options: _,
            xray_version,
        } => {
            format!(
                "xray-{}-{}-{}",
                xray_version, args.go_target.goarch, args.go_target.goos
            )
        }
    };
    let dir = TEMP_DIR.join(dir_name);
    std::fs::create_dir(&dir)?;
    log::debug!(
        "Created directory: {}. All files will be copied to it.",
        dir.display()
    );

    let mut files = Vec::new();
    files.push(repo_dir.join({
        let mut s: String;
        match args.target {
            CompileTarget::V2ray {
                compile_options: _,
                v2ray_version: _,
            } => s = String::from("v2ray"),
            CompileTarget::Xray {
                compile_options: _,
                xray_version: _,
            } => s = String::from("xray"),
        };
        if args.go_target.goos == "windows" {
            s.push_str(".exe");
        }
        s
    }));
    files.push(repo_dir.join("README.md"));
    files.push(repo_dir.join("LICENSE"));
    files.push(TEMP_DIR.join("geoip.dat"));
    files.push(TEMP_DIR.join("geosite.dat"));
    if args.go_target.goos == "windows" {
        files.push(TEMP_DIR.join("wintun.dll"));
        files.push(TEMP_DIR.join("LICENSE-wintun.txt"));
    }

    log::debug!("Files that will be packaged: {files:#?}");
    for ref file in files {
        let dest = dir.join(file.file_name().unwrap());
        log::debug!("Copying {} to {}", file.display(), dest.display());
        std::fs::copy(file, &dest)?;
    }
    Ok(dir)
}

fn compress_zip() -> PackResult<PathBuf> {
    let args = ARGS.get().unwrap();
    let repo_dir = REPOSITORY_DIR.get().unwrap();
    let name = match &args.target {
        CompileTarget::V2ray {
            compile_options: _,
            v2ray_version,
        } => {
            format! {
                "v2ray-{}-{}-{}.zip",
                v2ray_version, args.go_target.goarch, args.go_target.goos,
            }
        }
        CompileTarget::Xray {
            compile_options: _,
            xray_version,
        } => {
            format!(
                "xray-{}-{}-{}.zip",
                xray_version, args.go_target.goarch, args.go_target.goos
            )
        }
    };

    let zip_path = TEMP_DIR.join(name);
    let file_writer = std::fs::File::create(&zip_path).unwrap();
    let mut zip_writer = zip::ZipWriter::new(file_writer);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    log::debug!(
        "Created zip file: {}. All files will be compressed into it.",
        zip_path.display()
    );

    // Get collected files and add README.md and LICENSE from repo
    let mut files = COLLECTED_FILES.lock().unwrap().clone();
    files.push(repo_dir.join("README.md"));
    files.push(repo_dir.join("LICENSE"));

    for ref file in files {
        log::debug!("Compressing {}", file.display());
        let mut file_reader = std::fs::File::open(file)
            .unwrap_or_else(|_| panic!("Failed to open {}", file.display()));

        let dest = file.file_name().unwrap().to_str().unwrap();
        zip_writer.start_file(dest, options)?;
        std::io::copy(&mut file_reader, &mut zip_writer)?;
    }

    log::info!("All files packaged.");
    Ok(zip_path)
}

pub fn package_all() -> PackResult<()> {
    let zip_path = compress_zip()?;

    // copy to target directory
    let args = ARGS.get().unwrap();
    std::fs::create_dir_all(&args.path_options.output_path).unwrap();
    let release_path = args
        .path_options
        .output_path
        .join(zip_path.file_name().unwrap());
    std::fs::copy(&zip_path, &release_path).unwrap_or_else(|_| {
        panic!(
            "Failed to copy {} to {}",
            zip_path.display(),
            release_path.display()
        )
    });

    log::info!("Copied the package to {}", release_path.display());
    Ok(())
}
