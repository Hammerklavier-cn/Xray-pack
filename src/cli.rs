use std::{fmt::Display, path::PathBuf, sync::LazyLock};

use clap::{Parser, Subcommand, ValueEnum};

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
+ `GOAMD64`, `GO386`, `GOARM`, `GOARM64`, etc. environment variables: Specify the microarchitecture of the CPU.
+ `GOEXPERIMENT` environment variable: Specify the experimental features of the Go compiler.
    `jsonv2`, `newinliner` might improve performance.
+ `CGO_ENABLED` environment variable: Specify whether to enable CGO. `CGO_ENABLED=0` is recommended for better performance. \
    Note that this programme will override `go env CGO_ENABLED` and set `CGO_ENABLED=0` unless you specify it manually.
+ `--gcflags`: Specify the `-gcflags` for the Go compiler. Default to `all=-l=4`, maximizing inline optimization.\
"
)]
pub struct Args {
    #[command(flatten)]
    pub path_options: PathOptions,

    /// To build xray or v2ray
    #[command(subcommand)]
    pub target: CompileTarget,

    #[command(flatten)]
    pub go_target: GoTarget,

    #[command(flatten)]
    pub download_options: DownloadOptions,

    #[arg(short, long, default_value_t = false, help = "Enable verbose output")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum CompileTarget {
    Xray {
        #[command(flatten)]
        compile_options: XrayCompileOptions,

        #[arg(
            long,
            help = "Specify xray version (tag or branch)",
            default_value = "main"
        )]
        xray_version: String,
    },
    V2ray {
        #[command(flatten)]
        compile_options: XrayCompileOptions,

        #[arg(
            long,
            help = "Specify v2ray version (tag or branch)",
            default_value = "master"
        )]
        v2ray_version: String,
    },
}
impl Display for CompileTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileTarget::Xray { .. } => write!(f, "Xray-core"),
            CompileTarget::V2ray { .. } => write!(f, "v2ray-core"),
        }
    }
}
impl CompileTarget {
    const XRAY_CORE_REPO: &'static str = "https://github.com/XTLS/Xray-core.git";
    const V2RAY_CORE_REPO: &'static str = "https://github.com/v2fly/v2ray-core.git";

    pub fn repo_url(&self) -> &'static str {
        match self {
            CompileTarget::Xray { .. } => Self::XRAY_CORE_REPO,
            CompileTarget::V2ray { .. } => Self::V2RAY_CORE_REPO,
        }
    }

    pub fn repo_version(&self) -> String {
        match self {
            CompileTarget::V2ray {
                compile_options: _,
                v2ray_version,
            } => v2ray_version.clone(),
            CompileTarget::Xray {
                compile_options: _,
                xray_version,
            } => xray_version.clone(),
        }
    }
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
pub struct XrayCompileOptions {
    #[arg(long, help = "-gcflags for Go compiler", default_value = "all:-l=4")]
    pub gcflags: String,

    #[arg(
        long,
        help = "-ldflags for Go compiler. \
                Default is `-X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=`, \
                with `COMMID` being the commit hash of the source code."
    )]
    pub ldflags: Option<String>,
}

#[derive(Debug, Parser)]
pub struct V2rayCompileOptions {
    #[arg(
        long,
        help = "-gcflags for Go compiler.",
        long_help = "-gcflags for Go compiler. \
                     Note that the default value of v2ray's official release is to leave it empty. \
                     However, inline is highly recommended for better performance.",
        default_value = "all:-l=4"
    )]
    pub gcflags: String,

    #[arg(
        long,
        help = "-ldflags for Go compiler. \
                Default is `-s -w -buildid=`."
    )]
    pub ldflags: Option<String>,
}

#[derive(Debug, Parser)]
pub struct GoTarget {
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
