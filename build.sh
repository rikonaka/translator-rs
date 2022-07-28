#!/bin/bash

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu