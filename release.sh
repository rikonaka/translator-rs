#!/bin/bash

echo "DO NOT USE IT IN WINDOWS!!!"

rm ./translator-rs-x86_64-unknown-linux-musl.zip

zip -j ./translator-rs-x86_64-unknown-linux-musl.zip ./target/x86_64-unknown-linux-musl/release/translator-rs