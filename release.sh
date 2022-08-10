#!/bin/bash

rm ./target/rust-translator-aarch64-unknown-linux-gnu.zip
rm ./target/rust-translator-x86_64-pc-windows-gnu.zip
rm ./target/rust-translator-x86_64-unknown-linux-gnu.zip
rm ./target/rust-translator-x86_64-unknown-linux-musl.zip

zip ./target/rust-translator-aarch64-unknown-linux-gnu.zip ./target/aarch64-unknown-linux-gnu/release/rust-translator
zip ./target/rust-translator-x86_64-pc-windows-gnu.zip ./target/x86_64-pc-windows-gnu/release/rust-translator.exe
zip ./target/rust-translator-x86_64-unknown-linux-gnu.zip ./target/x86_64-unknown-linux-gnu/release/rust-translator
zip ./target/rust-translator-x86_64-unknown-linux-musl.zip ./target/x86_64-unknown-linux-musl/release/rust-translator