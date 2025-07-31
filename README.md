# Media File Date Fixer (mfdf)

**mfdf** restores lost Created and Modfied dates in most popular photo and video files. This often occurs after copying media files from SD cards, tablets, phones, etc to another device for editing or viewing. The code uses [nom-exif](https://github.com/mindeng/nom-exif), so go there to see the currently supported file types.

The repo consists of a Rust library with simple MacOS (Swift) and Windows (C# WinUI 3) front end applications. For other operating systems, there is a command line version which can be built thus:

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `git clone https://github.com/bobosola/media-file-date-fixer`
3. `cd media-file-date-fixer`
4. `cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` in Windows). It takes a directory path as its single argument, e.g. `./mfdf /Users/bob/myvideos`

The code recursively scans the chosen directory. It attempts to retrieve metadata from each supported file type, and uses that data to update the file's `Created` and `Modified` dates.

NB: it can retrieve both the `Created` and `Modified` dates for images, but only the `Created` date for videos because `Modified` dates are not available in video metadata. So for videos, the `Created` date is also used to update the `Modified` date as there is usually no significant difference between them anyway.

Note that Linux only supports altering the `Modified` date ([more details here](https://www.figuiere.net/technotes/notes/tn005/)).

On completion, the code returns a summary report containing a count of successes, ignored files, and any errors.
