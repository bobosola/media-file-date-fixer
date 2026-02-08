# Media File Date Fixer (mfdf)

**mfdf** restores lost `Created` dates in most popular photo and video files. These dates often get overwritten when copying files around from SD cards, tablets, and phones. Running **mfdf** on a directory of copied media files will restore the original `Created` date of all the supported files types, including files in any subdirectories. The code uses [nom-exif](https://github.com/mindeng/nom-exif), so check there to see the currently supported file types.

The repo consists of:
* a Rust library (`mfdf/src/lib.rs`) containing the core logic
* a command line app (`mfdf/src/main.rs`) for systems which support Rust
* a macOS (Swift) front end (see README in `mac_app` for how to build) with `ffi_lib` compiled as a `dylib`
* a Windows (C# WinUI 3) front end (see README in `win_app` for how to build) with the `ffi_lib` compiled as a `dll`

To build the command line app, you will need to have [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com) installed. Then do this:
1. `git clone https://github.com/bobosola/media-file-date-fixer`
2. `cd media-file-date-fixer`
3. `cargo build --release`

The CLI app should then be available in `target/release` as `mfdf` (or `mfdf.exe` on Windows). It takes a directory path as its single argument.

## Quick Warning
**mfdf** is very unlikely to work on media files downloaded from social media sites as these typically have their metadata stripped for privacy reasons. Also, some editing applications have been known to strip metadata. So please ensure that you are working on copies of your original media files as a safety measure as there is no Undo feature. 

## Example Usage

```bash
# Basic usage
./mfdf /path/to/media/files

# Path with spaces
./mfdf "/path/to/my photos"
```

The three app types are functionally equivalent. They all recursively scan the chosen directory and attempt to retrieve `Created` date metadata from each supported file type, then:

 - On Macs and Windows, the app uses the found date to update the file's OS `Created` date. 
 - On other Unix-like systems (Linux etc.), the found date is used to update the file's OS `Modified` date as a better-than-nothing solution because the `Created` date on such systems is strictly read-only. 

**Note:** After running **mfdf**, the updated OS date and time in a copied file will show the same time of day that the media file was created. In other words, wall clock time is maintained rather than UTC time. For example, a shot taken in Bermuda at 16:00 with the camera set to local time will still show as 16:00 on a copied file made in London. Metadata time zone offsets are deliberately ignored if supplied, otherwise the London file copy would show as having been created at 20:00, which is arguably correct but not helpful for viewing files in shot date order if mixed with other media files from the same location which have no time zone offset supplied (which is very common).

On completion, the code returns a summary report containing a count of the number of files examined, a count of the number of files updated, and a count of failed files with error details of any failed files.
