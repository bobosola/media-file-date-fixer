# MacOS Build #

Build requirements:
* Xcode 16.4 (but should build on earlier versions)
* Rust 2024 (v1.85.0 or higher)

This is an XCode MacOS desktop app in Swift. It requires the `mfdf_ffi.dylib` to be built in Rust. Once Rust is installed, the dylib can be built by running `release.sh` which builds both Intel and Mac silicon release version dylibs and combines them into a Universal Binary dylib created in the Xcode project directory (NB: check the shell path before running).
