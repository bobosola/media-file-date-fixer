# Media File Date Fixer (mfdf)

mfdf restores lost Created and Modfied dates in most popular photo and video files. This often occurs after copying media files from SD cards, tablets, phones, etc to another device for editing or viewing. The code uses [nom-exif](https://github.com/mindeng/nom-exif), so go there to see the currently supported file types.

The repo consists of a Rust library with simple MacOS and Windows front end applications. There is also a command line app which can be built thus:

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `git clone https://github.com/bobosola/media-file-date-fixer`
3. `cd media-file-date-fixer`
4. `cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` in Windows). It takes a directory path as its single argument, e.g.
`./mfdf /Users/bob/myvideos`

The code recursively scans a directory containing supported media files. It retrieves metadata from each file, and uses that data to update the file's `Created` and `Modified` dates. It can retrieve both the `Created` and `Modified` dates for images, but only the `Created` date for videos because that is not available in video metadata. So for videos, the `Created` date is also used to populate the `Modified` date (there is usually no significant difference between them anyway).

Note that Linux only supports altering the `Modified` date only ([more details here](https://www.figuiere.net/technotes/notes/tn005/)).

On completion, the code returns a summary report of successes, failures and errors.
