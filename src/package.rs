use std::path::PathBuf;

use crate::{ARGS, REPOSITORY_DIR, TEMP_DIR, errors::PackResult};

#[deprecated(note = "Use compress_zip instead.")]
#[allow(dead_code)]
fn copy_to_dir() {
    let args = ARGS.get().unwrap();
    let repo_dir = REPOSITORY_DIR.get().unwrap();

    let name = format!(
        "Xray-{}-{}-{}",
        args.xray_version, args.build_options.goarch, args.build_options.goos
    );
    let dir = TEMP_DIR.join(name);
    std::fs::create_dir(&dir).unwrap();
    log::debug!(
        "Created directory: {}. All files will be copied to it.",
        dir.display()
    );

    let mut files = Vec::new();
    files.push(repo_dir.join(if args.build_options.goos == "windows" {
        "xray.exe"
    } else {
        "xray"
    }));
    files.push(repo_dir.join("README.md"));
    files.push(repo_dir.join("LICENSE"));
    files.push(TEMP_DIR.join("geoip.dat"));
    files.push(TEMP_DIR.join("geosite.dat"));
    if args.build_options.goos == "windows" {
        files.push(TEMP_DIR.join("wintun.dll"));
        files.push(TEMP_DIR.join("LICENSE-wintun.txt"));
    }

    log::debug!("Files that will be packaged: {files:#?}");
    for ref file in files {
        let dest = dir.join(file.file_name().unwrap());
        log::debug!("Copying {} to {}", file.display(), dest.display());
        std::fs::copy(file, &dest).unwrap();
    }
}

fn compress_zip() -> PackResult<PathBuf> {
    let args = ARGS.get().unwrap();
    let repo_dir = REPOSITORY_DIR.get().unwrap();
    let name = format!(
        "Xray-{}-{}-{}.zip",
        args.xray_version, args.build_options.goarch, args.build_options.goos
    );

    let zip_path = TEMP_DIR.join(name);
    let file_writer = std::fs::File::create(&zip_path).unwrap();
    let mut zip_writer = zip::ZipWriter::new(file_writer);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    log::debug!(
        "Created zip file: {}. All files will be compressed into it.",
        zip_path.display()
    );

    let mut files = Vec::new();
    files.push(TEMP_DIR.join(if args.build_options.goos == "windows" {
        "xray.exe"
    } else {
        "xray"
    }));
    files.push(repo_dir.join("README.md"));
    files.push(repo_dir.join("LICENSE"));
    files.push(TEMP_DIR.join("geoip.dat"));
    files.push(TEMP_DIR.join("geosite.dat"));
    if args.build_options.goos == "windows" {
        files.push(TEMP_DIR.join("wintun.dll"));
        files.push(TEMP_DIR.join("LICENSE-wintun.txt"));
    }

    for ref file in files {
        log::debug!("Compressing {}", file.display());
        let mut file_reader =
            std::fs::File::open(file).expect(&format!("Failed to open {}", file.display()));

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
    std::fs::create_dir_all(&args.output_path).unwrap();
    let release_path = args.output_path.join(zip_path.file_name().unwrap());
    std::fs::copy(&zip_path, &release_path).expect(&format!(
        "Failed to copy {} to {}",
        zip_path.display(),
        release_path.display()
    ));

    log::info!("Copied the package to {}", release_path.display());
    Ok(())
}
