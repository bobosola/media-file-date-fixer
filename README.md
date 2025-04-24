# Media File Date Fixer

A common problem when copying media files from SD cards is that the original Created and Modified dates can be overridden on the copied files. This happens because copy operations often insert the date and time when the copy operation was performed, usually resulting in all the copied files showing the same date and time.

This repo fixes that problem for the file formats listed below by recursively scanning a directory and using any found file metadata timestamps (EXIF data etc.) to correct the copied file dates. The Created and Modified dates can usually be retrieved for image files, but only a Created date is retrievable for video and audio file formats.

Note that file metadata can be missing for several reasons, e.g. if the file was retrieved from an online service that strips EXIF data. So the code works best with media files copied directly from a camera.

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
- a count of files with errors (unsupported file types, permission problems, or with missing or damaged metadata)
- an errors list with error descriptions and the file path of the problem files
