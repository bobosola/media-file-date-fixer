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

# Create temporary copies with fixed install names for each architecture
# The install name must be @rpath/libmfdf.dylib so dyld can find it at runtime
cp target/aarch64-apple-darwin/release/libmfdf_ffi.dylib /tmp/libmfdf_arm64.dylib
cp target/x86_64-apple-darwin/release/libmfdf_ffi.dylib /tmp/libmfdf_x86_64.dylib

# Fix the install name for both architectures
install_name_tool -id "@rpath/libmfdf.dylib" /tmp/libmfdf_arm64.dylib
install_name_tool -id "@rpath/libmfdf.dylib" /tmp/libmfdf_x86_64.dylib

# Create the fat binary dylib in the target directory with the correct install name
lipo \
/tmp/libmfdf_arm64.dylib \
/tmp/libmfdf_x86_64.dylib \
-output mac_app/mfdf/mfdf/libmfdf.dylib -create

# Clean up temp files
rm -f /tmp/libmfdf_arm64.dylib /tmp/libmfdf_x86_64.dylib

# Verify the install name is correct
echo "Checking install name:"
otool -D mac_app/mfdf/mfdf/libmfdf.dylib

# Optional: check the fat dylib contains both architectures
# - should return: 'x86_64 arm64'
echo "Checking architectures:"
lipo -archs mac_app/mfdf/mfdf/libmfdf.dylib
