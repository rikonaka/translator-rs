#!/bin/bash

echo "DO NOT USE IT IN WINDOWS!!!"

rm ./target/translator-rs-x86_64-unknown-linux-musl.zip
# rm ./target/translator-rs-x86_64-unknown-linux-gnu.zip
# rm ./target/translator-rs-aarch64-unknown-linux-gnu.zip
# rm ./target/translator-rs-x86_64-pc-windows-gnu.zip

zip -j ./target/translator-rs-x86_64-unknown-linux-musl.zip ./target/x86_64-unknown-linux-musl/release/translator-rs
# zip -j ./target/translator-rs-x86_64-unknown-linux-gnu.zip ./target/x86_64-unknown-linux-gnu/release/translator-rs
# zip -j ./target/translator-rs-aarch64-unknown-linux-gnu.zip ./target/aarch64-unknown-linux-gnu/release/translator-rs
# zip -j ./target/translator-rs-x86_64-pc-windows-gnu.zip ./target/x86_64-pc-windows-gnu/release/translator-rs.exe