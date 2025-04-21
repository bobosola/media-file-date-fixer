# Media File Date Fixer

A common problem when copying media files from SD cards to a computer is that the original Created and Modified dates for the media can be overridden. This happens because copy operations often ignore the original dates, and instead insert the date and time when the copy operation was performed. This results in all the copied files showing the same date and time.

This application and library fixes that problem for the media formats listed below by:

- scanning a user-chosen parent directory (and all sub-directories)
- reading the metadata contained within each supported file type
- obtaining the original Created and Modified dates from the metadata
- updating the OS inode dates (or Windows MFT dates) with the metadata dates to effectively recreate the original file dates as seen in a directory listing or file explorer.

This enables a mix of media files with different naming conventions to be ordered chronologically for sequential viewing or editing.

The repo uses [nom-exif](https://github.com/mindeng/nom-exif) for the file parsing. So the file types supported here are the same as the file types supported by that crate, which are currently:

- Image
  - *.heic, *.heif, etc.
  - *.jpg, *.jpeg
  - *.tiff, *.tif
  - *.RAF (Fujifilm RAW)
- Video/Audio
  - ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp, etc.
  - Matroska based file format: *.webm, *.mkv, *.mka, etc.

The library returns a summary report containing:
- a count of the files examined
- a count of the files updated
- an errors list with the error description and file paths of the problem files

Note that EXIF data can be missing from an image for several reasons, including if the image was edited in a way that removes the metadata, if the image was taken from a non-digital source like a scanned print, or if the image was uploaded to a service that strips EXIF data.

Note also that for video files, the [nom-exif](https://github.com/mindeng/nom-exif) code only attempts to read and update the Created date. So only the Created date is updated in copied video files, and the Modifed data remains untouched.
