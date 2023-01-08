#!/bin/bash

echo "DO NOT USE IT IN WINDOWS!!!"

# mv ./.cargo/config.toml.bak ./.cargo/config.toml
echo "build target x86_64-unknown-linux-musl"
cargo build --release --target x86_64-unknown-linux-musl
echo "build target x86_64-unknown-linux-gnu"
cargo build --release --target x86_64-unknown-linux-gnu
echo "build target aarch64-unknown-linux-gnu"
cargo build --release --target aarch64-unknown-linux-gnu
echo "build target x86_64-pc-windows-gnu"
cargo build --release --target x86_64-pc-windows-gnu
# run at the windows
# cargo build --release --target x86_64-pc-windows-msvc
# mv ./.cargo/config.toml ./.cargo/config.toml.bak