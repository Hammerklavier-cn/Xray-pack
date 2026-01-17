# Xray packager

Build and package Xray (core of Project X) on your machine.

This compilation script empowers you the ability to customise build options so that you can fully release the performance of not only Xray-core but also your machine by specifying go compiler flags (`-gcflags`), linker flags (`-ldflags`), cpu architecture (`GOAMD64`, `GO386`, `GOARM`, `GOMIPS`, `GOMIPS64`, `GOPPC64` and `GORISCV64` environment variables), `CGO_ENABLED` (set to 0 to disable CGO, which might improve performance), and `GOEXPERIMENT` (`jsonv2`, `newinliner`, `greenteagc`, `regabiargs`, `regabiwrappers` might improve performance).

## Usage

Enable all features for x86_64 cpu:

```bash
CGO_ENABLED=0 GOAMD64="v4" GOEXPERIMENT="greenteagc,jsonv2,newinliner,regabiargs,regabiwrappers" ./Xray-pack.exe -p ../Xray-core/ -v --goos linux --goarch amd64
```

Detailed usage:

```text
Usage: Xray-pack.exe [OPTIONS]

Options:
  -s, --from-source
          Build Xray-core from source code. The repository will be downloaded to the current directory (C:\msys64\home\q5vsx\project\Xray-pack).

  -p, --source-path <SOURCE_PATH>
          Path to the source code directory.

          [default: C:\msys64\home\q5vsx\project\Xray-pack]

      --xray-version <XRAY_VERSION>
          Specify Xray-core version (tag or branch)

          [default: main]

  -o, --output-path <OUTPUT_PATH>
          Output destination directory.

          [default: dist]

      --goos <GOOS>
          Specify GOOS for the Go compiler. Default to `linux`

          [default: linux]

      --goarch <GOARCH>
          Specify GOARCH for the Go compiler. Default to `amd64`

          [default: amd64]

      --gcflags <GCFLAGS>
          gcflags for Go compiler

          [default: all:-l=4]

      --ldflags <LDFLAGS>
          ldflags for Go compiler. Default is `-X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=`, with `COMMID` being the commit hash of the source code.

      --region <REGION>
          Specify region for geo files

          [default: china-mainland]
          [possible values: china-mainland, russia, iran]

  -v, --verbose
          Enable verbose output

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
