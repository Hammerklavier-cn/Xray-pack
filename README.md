# Xray-pack

[中文](README_zh.md)

A Rust-based tool to build and package [Xray-core](https://github.com/XTLS/Xray-core) and [V2Ray-core](https://github.com/v2fly/v2ray-core) with maximum flexibility and performance tuning.

## Features

- **Dual Core Support**: Supports building both Xray-core and v2ray-core.
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

## Build & Installation

Type in the terminal

```bash
CFLAGS="-O3 -march=native" CXXFLAGS="-O3 -march=native" RUSTFLAGS="-C target-cpu=native" cargo install --path .
```

to install the most optimized version of the programme.

## Usage

### Proxy

The programme determines the proxy settings from the environment variables in the following order: `HTTPS_PROXY`, `https_proxy`, `ALL_PROXY` and `all_proxy`.

### Command Line Options

```text
Usage: xray-pack.exe [OPTIONS] <COMMAND>

Commands:
  xray
  v2ray
  help   Print this message or the help of the given subcommand(s)

Options:
  -s, --from-source                Build Xray-core from source code. The repository will be downloaded to the current directory.
  -p, --source-path <SOURCE_PATH>  Path to the source code directory. [default: current directory]
  -o, --output-path <OUTPUT_PATH>  Output destination directory. [default: dist]
      --goos <GOOS>                Specify GOOS for the Go compiler. This will override `GOARCH` and `go env GOOS` values. [default: linux]
      --goarch <GOARCH>            Specify GOARCH for the Go compiler. This will override `GOARCH` and `go env GOARCH` values. [default: amd64]
      --region <REGION>            Specify region for geo files [default: china-mainland] [possible values: china-mainland, russia, iran]
  -v, --verbose                    Enable verbose output
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version



Usage: xray-pack.exe xray [OPTIONS]

Options:
      --gcflags <GCFLAGS>            -gcflags for Go compiler [default: all:-l=4]
      --ldflags <LDFLAGS>            -ldflags for Go compiler. Default is `-X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=`, with `COMMID` being the commit hash of the source code.
      --xray-version <XRAY_VERSION>  Specify xray version (tag or branch) [default: main]
  -h, --help                         Print help



Usage: xray-pack.exe v2ray [OPTIONS]

Options:
      --gcflags <GCFLAGS>              -gcflags for Go compiler [default: all:-l=4]
      --ldflags <LDFLAGS>              -ldflags for Go compiler. Default is `-X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=`, with `COMMID` being the commit hash of the source code.
      --v2ray-version <V2RAY_VERSION>  Specify v2ray version (tag or branch) [default: master]
  -h, --help                           Print help
```

### Examples

Enable all features for x86_64 CPU and Linux system (Only use GOAMD=v4 if CPU supports AVX512 instructions):

```bash
CGO_ENABLED=0 GOAMD64="v4" GOEXPERIMENT="greenteagc,jsonv2,newinliner" -s -v --goos linux --goarch amd64 ./xray-pack.exe xray
```

Optimize for most x86_64 CPU (with AVX2 support) and Windows system:

```bash
CGO_ENABLED=0 GOAMD64="v3" ./xray-pack.exe -s -v --goos windows --goarch amd64 xray
```

Build for ARM64 MacOS, disabling inlining:

```bash
CGO_ENABLED=0 ./xray-pack.exe -s -v --goos darwin --goarch arm64 xray --gcflags "all:-l"
```

Build V2Ray with custom version:

```bash
CGO_ENABLED=0 ./xray-pack.exe -s -v --goos linux --goarch amd64 v2ray --v2ray-version v5.44.1
```

### Output

The packaged zip will be named:

For Xray:
```
xray-{version}-{arch}-{system}.zip
```

For V2Ray:
```
v2ray-{version}-{arch}-{system}.zip
```

and will include:

- Compiled binary (`xray`/`xray.exe` for Xray, `v2ray`/`v2ray.exe` for V2Ray)
- `geoip.dat` and `geosite.dat`
- `README.md` and `LICENSE`
- (Windows and Xray only) `wintun.dll` and `LICENSE-wintun.txt`

## License

This project is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
