#!/opt/homebrew/bin/bash

# Builds ARM & Intel versions of the dylib and a CLI test runner for each.
# It then combines the dylibs into a single fat binary dylib
# which is created in the Xcode project directory for use by the app

# Build for ARM64 macOS
cargo build --target=aarch64-apple-darwin --release

# Build for Intel 64-bit macOS
cargo build --target=x86_64-apple-darwin --release

# Create the dylib target directory in the Xcode project
mkdir -p mac_app/mfdf/mfdf

# Create the fat binary dylib in the target directory
lipo \
target/aarch64-apple-darwin/release/libmfdf_ffi.dylib \
target/x86_64-apple-darwin/release/libmfdf_ffi.dylib \
-output mac_app/mfdf/mfdf/libmfdf.dylib -create

# Optional: check the fat dylib contains both architectures
# - should return: 'x86_64 arm64'
lipo -archs mac_app/mfdf/mfdf/libmfdf.dylib
