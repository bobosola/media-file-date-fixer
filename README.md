# Media File Date Fixer

A common problem when copying media files from SD cards is that the original *Created* and *Modified* dates can be overridden on the copied files. This happens because Copy operations often insert the date and time when the copy operation was performed, resulting in all the copied files showing the same date and time. This repo fixes that problem for the file formats listed below by recursively scanning a directory containing media, retrieving metadata from within each file, and using that data to correct the copied file dates.

Note that the *Created* and *Modified* dates can usually be retrieved for image files, but only a *Created* date is currently retrievable for video and audio file formats. Thus a mix of media file types in a directory can then be ordered by the *Created* date for sequential viewing or editing.

The code is most effective when working with media files which have been copied directly from a camera. This is because file metadata in other media files can be missing for reasons such as:
- some online services strip Exif data for privacy reasons
- some editing applications change or remove metadata
- metadata may be missing entirely, such as in scanned images

The repo uses the [nom-exif](https://crates.io/crates/nom-exif) crate for the file parsing, which currently supports:
- Image
  - *.heic, *.heif, etc.
  - *.jpg, *.jpeg
  - *.tiff, *.tif
  - *.RAF (Fujifilm RAW)
- Video/Audio
  - ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp, etc.
  - Matroska based file format: *.webm, *.mkv, *.mka, etc.

The library code returns a summary report containing:
- a count of the total number of files examined
- a count of the files where one or both of the dates were updated
- a count of files with errors (unsupported file types, permission problems, or with files missing or damaged metadata)
- an errors list with error descriptions and the file paths of the problem files
