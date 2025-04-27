# Media File Date Fixer

A common problem when copying media files from mixed sources (typically from SD cards) is that the original *Created* and *Modified* dates can often be overwritten on the copied files. This results in the copied files all showing the same date and time, thus making it impossible to order them by the *Created* date for sequential viewing or editing. This repo fixes that problem for the file formats listed below:

- Image
  - *.heic, *.heif, etc.
  - *.jpg, *.jpeg
  - *.tiff, *.tif
  - *.RAF (Fujifilm RAW)
- Video/Audio
  - ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp, etc.
  - Matroska based file format: *.webm, *.mkv, *.mka, etc.

It recursively scans a directory containing supported media files, retrieves metadata from within each file, and uses that data to correct the copied file dates. Both *Created* and *Modified* dates can be retrieved for image files, but only a *Created* date is currently retrievable for video and audio file formats.

Note that file metadata can be missing from processed media files (i.e. downloaded from social media sites or after editing in software), so this code is most effective when working with files which have been copied directly from a camera. On completion, it returns a summary report containing:
- a count of the total number of files examined
- a count of the files where one or both of the dates were updated
- a count of files with errors (unsupported file types, permission problems, or with files missing or damaged metadata)
- an errors list with error descriptions and the file paths of the problem files

The code is built using:
- [walkdir](https://github.com/BurntSushi/walkdir) for fast recursive directory traversal
- [nom-exif](https://github.com/mindeng/nom-exif) for the file metadata parsing
- [chrono](https://github.com/chronotope/chrono) for the date and time handling
- [pathdiff](https://github.com/Manishearth/pathdiff) for relative path handling
