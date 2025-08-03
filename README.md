# Media File Date Fixer (mfdf)

**mfdf** restores lost Created and Modfied dates in most popular photo and video files. These dates often get overwritten when copying files around from SD cards, tablets, and phones. Running mfdf on a directory of copied media files will restore one or both of the original dates of all the supported files types, including files in any subdirectories. The code uses [nom-exif](https://github.com/mindeng/nom-exif), so check there to see the currently supported file types.

The repo consists of:
* a Rust library containing the core logic
* a command line app for systems which support Rust
* a MacOS (Swift) front end (see README in `mac_app` for how to build)
* a Windows (C# WinUI 3) front end (see README in `win_app` for how to build)

See the [mfdf website](https://mfdf.osola.org.uk) for more information and details of how to download
installable binaries for MacOS and Windows.

Build the command line app like this:
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `git clone https://github.com/bobosola/media-file-date-fixer`
3. `cd media-file-date-fixer`
4. `cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` on Windows). It takes a directory path as its single argument, e.g. `./mfdf /Users/bob/myvideos`

The 3 app types are functionally equivalent. They all recursively scan the chosen directory and attempt to retrieve metadata from each supported file type. The dates retrieved are as follows:
- Photos: `Created` and `Modified` dates
- Videos: `Created` date only (because a `Modified` date is not recorded in video metadata)

The apps then use that data to update the file's OS `Created` and/or `Modified` dates accordingly. OS support for altering dates in code looks like this:
- MacOS & Windows: `Created` and `Modified`
- Other Unix-like: `Modified` only

Other unix-like systems only support altering the `Modified` date because the `Created` date, known as `btime` (birth time), is strictly read-only and may even not exist on some old versions. So for video files on these systems, the metadata `Created` date is used to update the OS `Modified` date. The rationale here is that for the target use case (copying camera files) no-one cares about a potential small difference between the two dates anyway, and any metadata date is better than having just the (useless) date of the copy operation.

On completion, the code returns a summary report string containing a count of successes, ignored files, and any errors.
