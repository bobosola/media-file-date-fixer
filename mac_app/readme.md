# MacOS Build #

Build requirements:
* Xcode 16.4 (but should build on earlier versions)
* Rust 2024 (v1.85.0 or higher)

This is an XCode MacOS desktop app in Swift. It requires the `mfdf_ffi.dylib` which can be built by running `release.sh` which builds both Intel and Mac silicon release version dylibs and combines them into a Universal Binary dylib created in the Xcode project directory (NB: check the shell path before running).

`codesign.sh` handles packaging the Universal Binary into a DMG file for distribution. It also:
* code-signs the executable and the DMG
* notarizes and staples the DMG

in order to satisfy Gatekeeper requirements for Macs running Catalina or later.

This requires membership of the Apple Developer Program and an Apple "Developer ID Application" certificate installed locally in the keychain. This set-up allows signing without having to embed the certificate details in the script. There are more details of how this all works in the script comments.
