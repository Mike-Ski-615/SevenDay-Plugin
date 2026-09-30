# SevenDay-Plugin

用 Rust 编译为 `wasm32-wasip2` 的 Pumpkin 服务端插件。

## 环境准备

```bash
# 1. 安装 C 工具链（编译宿主 proc-macro / build script 用）
sudo dnf install -y gcc glibc-devel

# 2. 添加 wasm 目标
rustup target add wasm32-wasip2
```

## 编译

```bash
cargo build --release
```

编译配置在 `.cargo/config.toml`，已指定目标为 `wasm32-wasip2`。

## 产物

```
target/wasm32-wasip2/release/seven_days.wasm
```

复制到根目录：

```bash
cp target/wasm32-wasip2/release/seven_days.wasm ./seven_days.wasm
```
