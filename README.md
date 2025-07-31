# Media File Date Fixer (mfdf)

**mfdf** restores lost Created and Modfied dates in most popular photo and video files. This often occurs after copying media files from SD cards, tablets, phones, etc. to another device for editing or viewing. The code uses [nom-exif](https://github.com/mindeng/nom-exif), so check there to see the currently supported file types.

The repo consists of a Rust library with:
* a simple MacOS (Swift) front end
* a simple Windows (C# WinUI 3) front end
* a command line version for all operating systems which support Rust

The command line version can be built thus:
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `git clone https://github.com/bobosola/media-file-date-fixer`
3. `cd media-file-date-fixer`
4. `cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` in Windows). It takes a directory path as its single argument, e.g. `./mfdf /Users/bob/myvideos`

The app recursively scans the chosen directory. It attempts to retrieve metadata from each supported file type, and uses that data to update the file's `Created` and `Modified` dates. Note that only the `Created` date is retrieved for video files because `Modified` dates are not recorded in video metadata. So for videos, the `Created` date is also used to update the `Modified` date. The rational here is that there is usually no significant difference between them anyway. Note also that Linux only supports altering the `Modified` date in all caes ([more details here](https://www.figuiere.net/technotes/notes/tn005/)).

On completion, the code returns a summary report containing a count of successes, ignored files, and any errors.
