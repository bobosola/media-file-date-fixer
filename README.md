# Media File Date Fixer

mfdf restores lost Created and Modfied dates in most popular photo and video files after copying media files from SD cards, tablets, phones, etc. It uses [nom-exif](https://github.com/mindeng/nom-exif), so go there to see the currently supported file types.

The code consists of Rust library with simple MacOS and Windows front end applications. There is also a Rust command line app which can be built thus:

`git clone https://github.com/bobosola/media-file-date-fixer
cd media-file-date-fixer
cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` in Windows). It takes a directory path as its single argument, e.g.
`./mfdf /Users/bob/myvideos`

The code recursively scans a directory containing supported media files. It retrieves metadata from each file, and uses that data to update the file's system dates. It can retrieve the `Created` and `Modified` dates for images, but only the `Created` date for videos because that is not available in video metadata.

OS support for altering dates in code looks like this:
- MacOS: `Created` and `Modified`
- Windows: `Created` and `Modified`
- Linux: `Modified` only ([more details](https://www.figuiere.net/technotes/notes/tn005/))

So Macs and Windows happily accept the corrected dates. But there's an obvious problem for fixing up video dates on Linux: The metadata can only supply the video `Created` date, but Linux only allows altering the `Modified` date. So for Linux, the code uses a video's `Created` metadata date to update the system `Modified` date, which seems like a reasonable compromise that's better than nothing.

On completion, the code returns a summary report containing:
- a count of the total number of files examined
- a count of the files in which the dates were updated
- a count of ignored files based on unsupported file suffixes
- a count of files with errors (permission problems or missing or partial metadata)
- an errors list with error descriptions and the file paths of the problem files
