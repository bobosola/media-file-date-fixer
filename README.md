# Media File Date Fixer

A common problem when copying media files from mixed sources (typically off SD cards from cameras, action cams, drones etc.) is that the original `Created` and `Modified` dates are overwritten on the copied files. This results in the copied files all showing the date and time when the copying was done, thus making it impossible to order them by date in a single directory for sequential viewing or editing. And because each file type usually has a different naming convention, a mixed file collection cannot be ordered by sequential names. This repo fixes that problem for the file formats listed below:

- Image
  - *.heic, *.heif, etc.
  - *.jpg, *.jpeg
  - *.tiff, *.tif
  - *.RAF (Fujifilm RAW)
- Video/Audio
  - ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp, etc.
  - Matroska based file format: *.webm, *.mkv, *.mka, etc.

The code recursively scans a directory containing supported media files. It retrieves metadata from each file, and uses that data to update the copied file's system dates. Both MacOS and Windows support changing a file's `Created` and `Modified` dates, but other Unix-like systems [currently only support](https://www.figuiere.net/technotes/notes/tn005/) changing a file's `Modified` date in code. Note also that while the code can retrieve `Created` and `Modified` dates from Exif data for image files, it can only retrieve a `Created` date from metadata for video and audio file formats. So for systems other than MacOS or Windows, a video's `Created` metadata date is used to update the system `Modified` date, which seems like a reasonable compromise to allow a mix of file types on all platforms be be ordered by one or the other dates.

On completion, it returns a summary report containing:
- a count of the total number of files examined
- a count of the files where one or both of the dates were updated
- a count of files with errors (unsupported file types, permission problems, or files with missing or damaged metadata)
- an errors list with error descriptions and the file paths of the problem files

> [!TIP]
> File metadata can be missing from processed media files (i.e. downloaded from social media sites or after editing in software), so this code is most effective when working with files which have been copied directly from a camera.
