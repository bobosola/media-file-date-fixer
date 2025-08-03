# Media File Date Fixer (mfdf)

**mfdf** restores lost Created and Modfied dates in most popular photo and video files. This often occurs after copying media files from SD cards, tablets, phones, etc. to another device for editing or viewing. The code uses [nom-exif](https://github.com/mindeng/nom-exif), so check there to see the currently supported file types.

The repo consists of a Rust library with:
* a simple MacOS (Swift) front end
* a simple Windows (C# WinUI 3) front end
* a command line version for systems which support Rust

Build the command line version like this:
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `git clone https://github.com/bobosola/media-file-date-fixer`
3. `cd media-file-date-fixer`
4. `cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` on Windows). It takes a directory path as its single argument, e.g. `./mfdf /Users/bob/myvideos`

The app recursively scans the chosen directory. It attempts to retrieve metadata from each supported file type, and uses that data to update the file's OS `Created` and `Modified` dates. Note that only the `Created` date is retrieved for video files because `Modified` dates are not recorded in video metadata. So for videos, the metadata `Created` date is also used to update the OS `Modified` date. The rational here is that there is no significant difference between them for camera files, and no-one cares anyway in this particular use case.

Note that Unix-like systems only support altering the `Modified` date in all cases because the Created date, known as btime (birth time) is strictly read-only. It does not even exist on some old versions and is not universally supported in many modern versions.

On completion, the code returns a summary report containing a count of successes, ignored files, and any errors.
