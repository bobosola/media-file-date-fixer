#!/opt/homebrew/bin/bash

# Builds a single fat binary dylib containing the ARM & Intel
# dylibs and copies it to the Mac folder for use in XCode

# Build for ARM64 macOS
cargo build --target=aarch64-apple-darwin --release

# Build for Intel 64-bit macOS
cargo build --target=x86_64-apple-darwin --release

# Create the "fat binary" dylib - NB: target dir must exist!
lipo \
target/aarch64-apple-darwin/release/libmfdf_ffi.dylib \
target/x86_64-apple-darwin/release/libmfdf_ffi.dylib \
-output mac_app/mfdf/mfdf/libmfdf.dylib -create

# Optional: check the fat dylib contains both architectures
# (should return: x86_64 arm64)
lipo -archs mac_app/mfdf/mfdf/libmfdf.dylib
