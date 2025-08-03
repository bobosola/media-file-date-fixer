# Windows Build #

Build requirements:
- Visual Studio 2022
- Rust 2024 (v1.85.0 or higher)

This is a Visual Studio 2022 WinUI 3 desktop app solution in C#. It requires the `mfdf_ffi.dll` which can be built by running the appropriate batch file below.

**NB: You will need to alter the path for PROJECT_DIR before running the batch files**

The batch files will build the following:
* mfdf_ffl.dll: a Win64 DLL version of the library
* mfdf.exe: a Win64 command line test runner for the library which takes a directory path as its single argument

The DLL will also be copied to the VS project folder named in the batch file.

* `bat/debug_build.bat` builds debug versions in `media-file-date-fixer/target/x86_64-pc-windows-msvc/debug`
* `bat/release_build.bat` builds release versions in `media-file-date-fixer/target\x86_64-pc-windows-msvc/release`
