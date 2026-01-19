# Xray-pack

[中文](README_zh.md)

A Rust-based tool to build and package [Xray-core](https://github.com/XTLS/Xray-core) (the core of Project X) with maximum flexibility and performance tuning.

## Features

- **Customizable Build**: Fine-tune Go compiler flags (`-gcflags`, `-ldflags`), CPU architecture (`GOAMD64`, `GO386`, `GOARM`, `GOMIPS`, `GOMIPS64`, `GOPPC64`, `GORISCV64`), and Go experimental features (`GOEXPERIMENT`).
- **Geo Data Download**: Automatically downloads the latest `geoip.dat` and `geosite.dat` for specified regions (China Mainland, Russia, Iran).
- **Wintun Support**: Downloads and packages Wintun driver for Windows builds.
- **Flexible Source**: Build from a local source path or clone from the official repository.
- **Minimum Dependencies**: No additional dependencies required apart from Go compiler.
- **Cross Platform**: Supports building for multiple platforms (Linux, Windows, macOS) and different architectures.
- **One-step Packaging**: Outputs a ready-to-use zip package for deployment.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install), if you want to build from source.
- [Go](https://go.dev/doc/install) (must be in your `PATH`)

## Installation

```bash
cargo build --release
```

The executable will be in `target/release/Xray-pack.exe`.

## Usage

Enable all features for x86_64 CPU and Linux system (Only use GOAMD=v4 if CPU supports AVX512 instructions):

```bash
CGO_ENABLED=0 GOAMD64="v4" GOEXPERIMENT="greenteagc,jsonv2,newinliner" ./Xray-pack.exe -s -v --goos linux --goarch amd64
```

### Command Line Options

```text
Usage: Xray-pack.exe [OPTIONS]

Options:
  -s, --from-source           Build Xray-core from source code (clone to current directory)
  -p, --source-path <PATH>    Path to Xray-core source code [default: current directory]
  -o, --output-path <PATH>    Output directory [default: dist]
      --xray-version <VER>    Xray-core version/tag/branch [default: main]
      --goos <GOOS>           Target OS for Go compiler [default: linux]
      --goarch <GOARCH>       Target architecture for Go compiler [default: amd64]
      --gcflags <FLAGS>       Go compiler gcflags [default: all:-l=4]
      --ldflags <FLAGS>       Go compiler ldflags [default: -X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=]
      --region <REGION>       Region for geo files [default: china-mainland] [possible: china-mainland, russia, iran]
  -v, --verbose               Enable verbose output
  -h, --help                  Print help
  -V, --version               Print version
```

### Output

The packaged zip will be named:

```
Xray-{version}-{arch}-{system}.zip
```

and will include:

- Compiled Xray binary (`xray` or `xray.exe`)
- `geoip.dat` and `geosite.dat`
- `README.md` and `LICENSE`
- (Windows only) `wintun.dll` and `LICENSE-wintun.txt`

## License

This project is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
