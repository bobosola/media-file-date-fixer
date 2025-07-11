:: Release build for 64 bit Windows
SET ARCH=x86_64-pc-windows-msvc
cargo build --target=%ARCH% --release