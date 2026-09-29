#!/usr/bin/env bash
# 构建 SevenDays 插件。
#
# 两个必须避开的坑（都已在本地实测确认）：
#
#   1) 项目路径含中文「我的世界」→ MinGW 的 ld 会报 `ld returned 5`（access denied）。
#      解法：把 target 目录指到纯 ASCII 路径。
#
#   2) 本机没装 MSVC C++ 工具链，默认 host 链接器会命中 Git 的 /usr/bin/link（Unix 的 link，
#      不是 MSVC 的 link.exe）→ 报 "extra operand … Try 'link --help'"。
#      解法：改用已安装的 GNU host 工具链（依赖 MinGW 的 gcc）。
#
# 用法：bash build.sh
set -euo pipefail
cd "$(dirname "$0")"

export CARGO_TARGET_DIR="C:/tmp/seven-days-target"
cargo +stable-x86_64-pc-windows-gnu build --release

echo
echo "产物: $CARGO_TARGET_DIR/wasm32-wasip2/release/SevenDays.wasm"
