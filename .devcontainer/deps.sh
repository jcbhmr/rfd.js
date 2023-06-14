#!/bin/sh
set -e
export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -y

# Linux
sudo apt-get install -y --no-install-recommends \
  libglib2.0-dev libatk1.0-dev libgtk-3-dev

# macOS
# zig (devcontainer)
# llvm (devcontainer)
# TODO: Fix "library not found for '-lobjc'"
rustup target add x86_64-apple-darwin

# Windows
cargo install cargo-xwin
rustup target add x86_64-pc-windows-gnu

sudo rm -rf /var/lib/apt/lists/*
