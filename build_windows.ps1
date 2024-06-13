echo "build target x86_64-pc-windows-msvc"
cargo build --release --target x86_64-pc-windows-msvc

echo "build target i686-pc-windows-msvc"
cargo build --release --target i686-pc-windows-msvc

$x86file = ".\translator-rs-x86_64-pc-windows-msvc.zip"
$i686file = ".\translator-rs-i686-pc-windows-msvc.zip"

if (Test-Path $x86file) {
    Remove-Item $x86file -Force
}
7z a -tzip $x86file .\target\x86_64-pc-windows-msvc\release\translator-rs.exe

if (Test-Path $i686file) {
    Remove-Item $i686file -Force
}
7z a -tzip $i686file .\target\i686-pc-windows-msvc\release\translator-rs.exe