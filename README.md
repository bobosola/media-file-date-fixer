# Media File Date Fixer

A common problem when copying media files from mixed sources (typically files on SD cards in cameras, action cams, drones etc.) is that the original `Created` and `Modified` dates are overwritten on the copied files. This results in the copied files all showing the date and time when the copying was done, thus making it impossible to order them by date in a single directory for sequential viewing or editing. And because each file type usually has a different naming convention, a mixed file collection ordered by file names is no help. This repo fixes that problem for the file formats listed below:

- Image
  - *.heic, *.heif, etc.
  - *.jpg, *.jpeg
  - *.tiff, *.tif
  - *.RAF (Fujifilm RAW)
- Video/Audio
  - ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp, etc.
  - Matroska based file format: *.webm, *.mkv, *.mka, etc.

The code recursively scans a directory containing supported media files. It retrieves metadata from each file, and uses that data to update the copied file's system dates. The dates it can retrieve are:
- for images: the `Created` and `Modified` dates
- for videos: the `Created` date only

OS support for altering dates in code looks like this:
- MacOS: `Created` and `Modified`
- Windows: `Created` and `Modified`
- Linux: `Modified` only ([more details](https://www.figuiere.net/technotes/notes/tn005/))

So Macs and Windows happily accept the corrected dates. But there's an obvious probem for fixing up video dates on Linux: The metadata can only supply the `Created` date, but Linux only allows altering the `Modified` date. So for Linux, the code uses a video's `Created` metadata date to update the system `Modified` date, which seems like a reasonable compromise (both have the same value anyway for files taken off a camera).

On completion, the code returns a summary report containing:
- a count of the total number of files examined
- a count of the files where one or both of the dates were updated
- a count of files with errors (unsupported file types, permission problems, or files with missing or damaged metadata)
- an errors list with error descriptions and the file paths of the problem files

> [!TIP]
> File metadata can be missing from processed media files (i.e. downloaded from social media sites or after editing in software), so this code is most effective when working with files which have been copied directly off a camera.
