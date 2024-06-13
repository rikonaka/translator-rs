# mv ./.cargo/config.toml.bak ./.cargo/config.toml
echo "build target x86_64-unknown-linux-musl"
cargo build --release --target x86_64-unknown-linux-musl
# echo "build target x86_64-unknown-linux-gnu"
# cargo build --release --target x86_64-unknown-linux-gnu
# echo "build target aarch64-unknown-linux-gnu"
# cargo build --release --target aarch64-unknown-linux-gnu

muslfile="./translator-rs-x86_64-unknown-linux-musl.zip"

if [ -f "$muslfile" ]; then
    rm $muslfile
fi
zip -j $muslfile ./target/x86_64-unknown-linux-musl/release/translator-rs