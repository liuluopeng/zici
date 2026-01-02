#!/bin/bash

# 确保脚本在出错时停止执行
set -e

echo "=== 开始构建流程 ==="

# 步骤 1: 安装 Rust 工具链
echo "1. 安装 Rust 工具链..."
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env

# 步骤 2: 安装 wasm32 目标
echo "2. 安装 wasm32 目标..."
rustup target add wasm32-unknown-unknown

# 步骤 3: 安装 wasm-pack
echo "3. 安装 wasm-pack..."
cargo install wasm-pack

# 步骤 4: 构建 Rust WASM 包
echo "4. 构建 Rust WASM 包..."
cd ../
wasm-pack build --target web

# 步骤 5: 安装 Vue 项目依赖
echo "5. 安装 Vue 项目依赖..."
cd www_vue
npm install --legacy-peer-deps

# 步骤 6: 生成 CNCHAR 静态数据
echo "6. 生成 CNCHAR 静态数据..."
npm run cnchar-prod

# 步骤 7: 构建 Vue 项目
echo "7. 构建 Vue 项目..."
npm run build

echo "=== 构建流程完成 ==="
