# Xray-pack

[English](README.md)

一个基于 Rust 的工具，用于灵活、高性能地编译和打包 [Xray-core](https://github.com/XTLS/Xray-core) 和 [V2Ray-core](https://github.com/v2fly/v2ray-core)。

## 特性

- **双核心支持**：支持 Xray-core 和 V2Ray-core。
- **可定制构建**：支持自定义 Go 编译参数（`-gcflags`、`-ldflags`）、CPU 架构（`GOAMD64`、`GO386`、`GOARM`、`GOMIPS`、`GOMIPS64`、`GOPPC64`、`GORISCV64`）、Go 实验特性（`GOEXPERIMENT`）。
- **自动下载 Geo 数据**：根据区域自动下载最新的 `geoip.dat` 和 `geosite.dat`（支持中国大陆、俄罗斯、伊朗）。
- **Wintun 支持**：Windows 构建自动下载并打包 Wintun 驱动。
- **灵活源码来源**：可从本地源码或官方仓库克隆编译。
- **最小依赖**：仅需 Go 编译器。
- **跨平台支持**：支持构建多个平台（Linux、Windows、macOS）和不同架构。
- **一键打包**：输出可直接部署的 zip 包。

## 依赖

- [Rust](https://www.rust-lang.org/tools/install)（如果你想要从源码编译）
- [Go](https://go.dev/doc/install)（需在 `PATH` 中）

## 编译&安装

命令行中输入：

```bash
CFLAGS="-O3 -march=native" CXXFLAGS="-O3 -march=native" RUSTFLAGS="-C target-cpu=native" cargo install --path .
```

以安装最优化的程序版本

## 使用方法

### 代理

程序按以下顺序从环境变量中确定代理设置：`HTTPS_PROXY`、`https_proxy`、`ALL_PROXY` 和 `all_proxy`。

### 命令行参数

```text
用法: xray-pack.exe [OPTIONS] <COMMAND>

子命令:
  xray
  v2ray
  help   打印此消息或给定子命令的帮助信息

选项:
  -s, --from-source                从源码构建 Xray-core。仓库将下载到当前目录。
  -p, --source-path <SOURCE_PATH>  源码目录路径 [默认: 当前目录]
  -o, --output-path <OUTPUT_PATH>  输出目标目录 [默认: dist]
      --goos <GOOS>                指定 Go 编译器的 GOOS。这将覆盖 `GOARCH` 和 `go env GOOS` 值。[默认: linux]
      --goarch <GOARCH>            指定 Go 编译器的 GOARCH。这将覆盖 `GOARCH` 和 `go env GOARCH` 值。[默认: amd64]
      --region <REGION>            指定 geo 文件区域 [默认: china-mainland] [可选值: china-mainland, russia, iran]
  -v, --verbose                    启用详细输出
  -h, --help                       打印帮助（使用 '--help' 查看更多）
  -V, --version                    打印版本



用法: xray-pack.exe xray [OPTIONS]

选项:
      --gcflags <GCFLAGS>            Go 编译器的 -gcflags 参数 [默认: all:-l=4]
      --ldflags <LDFLAGS>            Go 编译器的 -ldflags 参数。默认值为 `-X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=`，其中 `COMMID` 为源码的提交哈希。
      --xray-version <XRAY_VERSION>  指定 xray 版本（标签或分支） [默认: main]
  -h, --help                         打印帮助



用法: xray-pack.exe v2ray [OPTIONS]

选项:
      --gcflags <GCFLAGS>              Go 编译器的 -gcflags 参数 [默认: all:-l=4]
      --ldflags <LDFLAGS>              Go 编译器的 -ldflags 参数。默认值为 `-X github.com/xtls/xray-core/core.build=${COMMID} -s -w -buildid=`，其中 `COMMID` 为源码的提交哈希。
      --v2ray-version <V2RAY_VERSION>  指定 v2ray 版本（标签或分支） [默认: master]
  -h, --help                           打印帮助
```

### 示例

为 x86_64 CPU 和 Linux 系统启用所有性能特性（若 CPU 支持 AVX512 指令集）：

```bash
CGO_ENABLED=0 GOAMD64="v4" GOEXPERIMENT="jsonv2,newinliner" ./xray-pack.exe -s -v --goos linux --goarch amd64 xray
```

为主流 x86_64 CPU 和 Windows 系统启用指令集优化（CPU 只支持到 AVX2 指令集）：

```bash
CGO_ENABLED=0 GOAMD64="v3" ./xray-pack.exe -s -v --goos windows --goarch amd64 xray
```

为 ARM64 CPU 和 macOS 编译，禁用内联优化：

```bash
CGO_ENABLED=0 ./xray-pack.exe -s -v --goos darwin --goarch arm64 xray --gcflags "all:-l"
```

构建指定版本的 V2Ray：

```bash
CGO_ENABLED=0 ./xray-pack.exe -s -v --goos linux --goarch amd64 v2ray --v2ray-version v5.44.1
```

### 输出内容

最终打包的 zip 文件命名为：

对于 Xray：

```
xray-{version}-{arch}-{system}.zip
```

对于 V2Ray：

```
v2ray-{version}-{arch}-{system}.zip
```

包含以下内容：

- 编译后的可执行文件（`xray`/`xray.exe` 对应 Xray，`v2ray`/`v2ray.exe` 对应 V2Ray）
- `geoip.dat` 和 `geosite.dat`
- `README.md` 和 `LICENSE`
- （仅 Windows+Xray）`wintun.dll` 和 `LICENSE-wintun.txt`

## 许可证

本项目采用 GNU 通用公共许可证 v3.0，详见 [LICENSE](LICENSE)。
