# Xray-pack

[English](README.md)

一个基于 Rust 的工具，用于灵活、高性能地编译和打包 [Xray-core](https://github.com/XTLS/Xray-core)（Project X 核心）。

## 特性

- **可定制构建**：支持自定义 Go 编译参数（`-gcflags`、`-ldflags`）、CPU 架构（`GOAMD64`、`GO386`、`GOARM`、`GOMIPS`、`GOMIPS64`、`GOPPC64`、`GORISCV64`）、Go 实验特性（`GOEXPERIMENT`）。
- **自动下载 Geo 数据**：根据区域自动下载最新的 `geoip.dat` 和 `geosite.dat`（支持中国大陆、俄罗斯、伊朗）。
- **Wintun 支持**：Windows 构建自动下载并打包 Wintun 驱动。
- **灵活源码来源**：可从本地源码或官方仓库克隆编译。
- **一键打包**：输出可直接部署的 zip 包。

## 依赖

- [Rust](https://www.rust-lang.org/tools/install)
- [Go](https://go.dev/doc/install)（需在 `PATH` 中）

## 安装

```bash
cargo build --release
```

可执行文件位于 `target/release/xray-pack.exe`。

## 使用方法

为 x86_64 CPU 和 Linux 系统启用所有性能特性（若 CPU 支持 AVX512 指令集）：

```bash
CGO_ENABLED=0 GOAMD64="v4" GOEXPERIMENT="greenteagc,jsonv2,newinliner" ./xray-pack.exe -s -v --goos linux --goarch amd64
```

### 命令行参数

```text
用法: xray-pack.exe [OPTIONS]

选项:
  -s, --from-source           从源码构建 xray-core（克隆到当前目录）
  -p, --source-path <路径>    xray-core 源码路径 [默认: 当前目录]
  -o, --output-path <路径>    输出目录 [默认: dist]
      --xray-version <版本>   xray-core 版本/标签/分支 [默认: main]
      --goos <GOOS>           Go 编译目标操作系统 [默认: linux]
      --goarch <GOARCH>       Go 编译目标架构 [默认: amd64]
      --gcflags <参数>        Go 编译 gcflags [默认: all:-l=4]
      --ldflags <参数>        Go 编译 ldflags [默认: -X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=]
      --region <区域>         Geo 数据区域 [默认: china-mainland] [可选: china-mainland, russia, iran]
  -v, --verbose               输出详细日志
  -h, --help                  显示帮助
  -V, --version               显示版本
```

### 输出内容

最终打包的 zip 文件命名为：

```
xray-{version}-{arch}-{system}.zip
```

包含以下内容：

- 编译后的 xray 可执行文件（`xray` 或 `xray.exe`）
- `geoip.dat` 和 `geosite.dat`
- `README.md` 和 `LICENSE`
- （仅 Windows）`wintun.dll` 和 `LICENSE-wintun.txt`

## 许可证

本项目采用 GNU 通用公共许可证 v3.0，详见 [LICENSE](LICENSE)。
