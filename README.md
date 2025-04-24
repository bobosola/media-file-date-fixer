# Media File Date Fixer

A common problem when copying media files from SD cards is that the original Created and Modified dates can be overridden on the copied files and thus become lost . This happens because copy operations often ignore the original file dates, and instead insert the date and time when the copy operation was performed, which results in all the copied files showing the same date and time.

This repo fixes that problem for the file formats listed below by scanning a user-chosen parent directory (and all its sub-directories) and using any found metadata timestamps (EXIF etc.) to correct the copied file dates. This then enables a mix of copied media files with different naming conventions to be ordered chronologically for sequential viewing or editing.

It works best with files copied direclty from a recording device, because file metadata can be missing for several reasons, e.g. if an image was taken from a non-digital source like a scanned print, or if an image was retrieved from an online service that strips EXIF data.

The repo uses [walkdir](https://github.com/BurntSushi/walkdir) for the recursive directory scanning and [nom-exif](https://github.com/mindeng/nom-exif) for the file parsing. So the file types supported here are as per the file types supported by nom-exif, currently:

- Image
  - *.heic, *.heif, etc.
  - *.jpg, *.jpeg
  - *.tiff, *.tif
  - *.RAF (Fujifilm RAW)
- Video/Audio
  - ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp, etc.
  - Matroska based file format: *.webm, *.mkv, *.mka, etc.

The library returns a summary report containing:
- a count of the total number of files examined
- a count of the files updated
- a count of files with errors (i.e. unsupported file types or missing metadata)
- an errors list with the error description and file paths of the problem files
