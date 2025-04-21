# Media File Date Fixer

A common problem when copying media files from SD cards to a computer is that the original file Created and Modified dates can be overridden. This happens because the copy operation often ignores the original dates, and instead inserts the date and time when the copy operation was performed. This results in all the files showing the same date and time.

This application fixes the problem for the media formats listed below by:

- reading the metadata contained within each file in a given directory (and any sub-directories)
- using the Created and Modified dates found within the file metadata to recreate the original file Created and Modified dates.

Thus a mix of media files with different naming conventions can once again be ordered chronologically for sequential viewing or editing.

This repo uses [nom-exif](https://github.com/mindeng/nom-exif) for the file parsing, so file types supported here are the same as the file types supported by that crate, which are currently:

- Image
  - *.heic, *.heif, etc.
  - *.jpg, *.jpeg
  - *.tiff, *.tif
  - *.RAF (Fujifilm RAW)
- Video/Audio
  - ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp, etc.
  - Matroska based file format: *.webm, *.mkv, *.mka, etc.

Unsupported file types are ignored.

A summary report count of the files examined, files updated, and files ignored are returned on run completion. If a supported file type cannot be corrected because of some error condition (e.g. because the EXIF or other metadata data is missing), then the individual error description and associated file name will listed under the summary report.
