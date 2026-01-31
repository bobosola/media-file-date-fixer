# Media File Date Fixer (mfdf)

**mfdf** restores lost `Created` dates in most popular photo and video files. These dates often get overwritten when copying files around from SD cards, tablets, and phones. Running `mfdf` on a directory of copied media files will restore the original `Created` date of all the supported files types, including files in any subdirectories. The code uses [nom-exif](https://github.com/mindeng/nom-exif), so check there to see the currently supported file types.

The repo consists of:
* a Rust library containing the core logic
* a command line app for systems which support Rust
* a MacOS (Swift) front end (see README in `mac_app` for how to build)
* a Windows (C# WinUI 3) front end (see README in `win_app` for how to build)

To build the command line app, you will need to have [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com) installed. Then do this:
1. `git clone https://github.com/bobosola/media-file-date-fixer`
2. `cd media-file-date-fixer`
3. `cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` on Windows). It takes a directory path as its single argument, e.g. `./mfdf /Users/bob/myvideos`

The 3 app types are functionally equivalent. They all recursively scan the chosen directory and attempt to retrieve `Created` date metadata from each supported file type. 

**Note:** On Macs and Windows, the apps use the found metadata `Created` date to update the file's OS `Created` date. Other unix-like systems only support altering the `Modified` date. This is because the `Created` date, known as `btime` (birth time), is strictly read-only and may even not exist on some old OS versions. So for files on such systems, the metadata `Created` date is used to update the OS `Modified` date. The rationale here is that for the target use case (copying camera files) no-one cares about a potential difference between the two dates anyway, and any metadata date is better than having just the (useless) date of the copy operation.

On completion, the code returns a summary report string containing a count of the number of files examined, a count of the number of files updated, and a count of failed files. If the count of failed files is greater than zero, the details of each failed file are listed.
