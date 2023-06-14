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

# pushd "$(mktemp -d)"
# curl -fsSLO http://crosstool-ng.org/download/crosstool-ng/crosstool-ng-1.25.0.tar.xz
# tar -xzvf crosstool-ng-*.tar.xz
# cd crosstool-ng-*
# ./configure --prefix=/usr/local
# make
# sudo make install
# popd
rustup target add x86_64-apple-darwin

# Windows
cargo install cargo-xwin
rustup target add x86_64-pc-windows-gnu

sudo rm -rf /var/lib/apt/lists/*
