use std::{path::PathBuf, sync::LazyLock};

use clap::{Parser, ValueEnum};

pub static ROOT: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap());

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "\
Script to build and package Xray-core.

Xray-core will be compiled and packaged as `Xray-{version}-{arch}-{system}.zip`. \
You can install the package with the official installation script from <https://github.com/XTLS/Xray-install/raw/main/install-release.sh>.

There are several options to maximize the performance of Xray-core:
+ `GOAMD64` `GO386` `GOARM` environment variables: Specify the microarchitecture of the CPU.
+ `GOEXPERIMENT` environment variable: Specify the experimental features of the Go compiler.
    `jsonv2`, `newinliner`, `greenteagc`, `regabiargs`, `regabiwrappers` might improve performance.
+ `CGO_ENABLED` environment variable: Specify whether to enable CGO. `CGO_ENABLED=0` is recommended for better performance. \
Note that this programme will override `go env CGO_ENABLED` and set `CGO_ENABLED=0` unless you specify it manually.
+ `--gcflags`: Specify the `-gcflags` for the Go compiler. Default to `all=-l=4`, maximizing inline optimization.\
"
)]
pub struct Args {
    #[command(flatten)]
    pub path_options: PathOptions,

    #[arg(
        long,
        help = "Specify Xray-core version (tag or branch)",
        default_value = "main"
    )]
    pub xray_version: String,

    #[command(flatten)]
    pub compile_options: CompileOptions,

    #[command(flatten)]
    pub download_options: DownloadOptions,

    #[arg(short, long, default_value_t = false, help = "Enable verbose output")]
    pub verbose: bool,
}

#[derive(Debug, Parser)]
pub struct PathOptions {
    #[arg(
        short = 's',
        long,
        help = format!(
            "Build Xray-core from source code. The repository will be downloaded to the current directory ({}).",
            ROOT.display()
        ),
        default_value_t = false,
        conflicts_with = "source_path"
    )]
    pub from_source: bool,

    #[arg(
        short = 'p',
        long,
        help = "Path to the source code directory.",
        conflicts_with = "from_source",
        default_value = ROOT.to_str().unwrap()
    )]
    pub source_path: PathBuf,

    #[arg(
        short = 'o',
        long,
        help = "Output destination directory.",
        default_value = "dist"
    )]
    pub output_path: PathBuf,
}

#[derive(Debug, Parser)]
pub struct CompileOptions {
    #[arg(
        long,
        help = "Specify GOOS for the Go compiler. This will override `GOARCH` and `go env GOOS` values.",
        default_value = "linux"
    )]
    pub goos: String,

    #[arg(
        long,
        help = "Specify GOARCH for the Go compiler. This will override `GOARCH` and `go env GOARCH` values.",
        default_value = "amd64"
    )]
    pub goarch: String,

    #[arg(long, help = "gcflags for Go compiler", default_value = "all:-l=4")]
    pub gcflags: String,

    #[arg(
        long,
        help = "ldflags for Go compiler. \
                Default is `-X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=`, \
                with `COMMID` being the commit hash of the source code."
    )]
    pub ldflags: Option<String>,
}

#[derive(Debug, Parser)]
pub struct DownloadOptions {
    #[arg(
        long,
        help = "Specify region for geo files",
        default_value = "china-mainland"
    )]
    pub region: Region,
}

/// Three main restricted regions.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Region {
    ChinaMainland,
    Russia,
    Iran,
}
impl Region {
    /// Returns the URL for downloading the geoip and geodat files for the region.
    /// Add `geoip.dat` and `geosite.dat` to the URL for downloading.
    pub fn url(&self) -> &str {
        match self {
            Region::ChinaMainland => {
                "https://raw.githubusercontent.com/Loyalsoldier/v2ray-rules-dat/release/"
            }
            Region::Russia => {
                "https://raw.githubusercontent.com/runetfreedom/russia-v2ray-rules-dat/release/"
            }
            Region::Iran => {
                "https://raw.githubusercontent.com/Chocolate4U/Iran-v2ray-rules/release/"
            }
        }
    }
}
