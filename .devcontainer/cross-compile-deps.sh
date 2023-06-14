#!/bin/bash
set -e

cargo install cargo-xwin
rustup target add x86_64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
