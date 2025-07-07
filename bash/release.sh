#!/opt/homebrew/bin/bash

# Build a Universal 64 bit binary for Macs

# Build for ARM64 macOS (11.0+, Big Sur+)
cargo build --target=aarch64-apple-darwin --release

# Build for intel 64-bit macOS (10.7+, Lion+)
cargo build --target=x86_64-apple-darwin --release

# Create the "fat binary" from the builds
# NB: the directories below must all exist!
lipo -create -output target/universal/mfdf \
target/aarch64-apple-darwin/release/mfdf \
target/x86_64-apple-darwin/release/mfdf

# Check the fat binary contains both executables
file target/universal/mfdf
