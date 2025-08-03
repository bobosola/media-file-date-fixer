use std::fs:: {File, FileTimes };
use std::fmt;
use std::time:: SystemTime;
use std::path::{ Path };
use std::error::Error;
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use pathdiff:: diff_paths;
use chrono:: {DateTime, FixedOffset };

/// Summary report of application run
pub struct Report {
    pub examined: i32,
    pub updated: i32,
    pub ignored: i32,
    pub failed: i32,
    pub err_msgs: Vec<String>
}
impl Default for Report {
    fn default() -> Self {
        return Report {
            examined: 0,
            updated: 0,
            ignored: 0,
            failed: 0,
            err_msgs: vec![]
        }
    }
}

// Holds any datetimes retrieved from metadata
struct DateTimes {
    created_date: Option<DateTime<FixedOffset>>,
    modified_date: Option<DateTime<FixedOffset>>
}
impl Default for DateTimes {
    fn default() -> Self {
        return DateTimes {
            created_date: None,
            modified_date: None
        }
    }
}

// Custom error type
#[derive(Debug)]
enum DateFixError {
    MissingDates,
    MissingMetadata,
    IoError(std::io::Error),
    ParseError(String),
}
impl fmt::Display for DateFixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateFixError::MissingDates => write!(f, "No dates found in metadata"),
            DateFixError::MissingMetadata => write!(f, "Missing metadata"),
            DateFixError::IoError(e) => write!(f, "IO error: {}", e),
            DateFixError::ParseError(msg) => write!(f, "{}", msg),
        }
    }
}
impl Error for DateFixError {}
impl From<std::io::Error> for DateFixError {
    fn from(error: std::io::Error) -> Self {
        DateFixError::IoError(error)
    }
}
impl From<nom_exif::Error> for DateFixError {
    fn from(error: nom_exif::Error) -> Self {
        DateFixError::ParseError(error.to_string())
    }
}

/// Attempts to fix lost Created & Modified dates in common media files
/// by recovering the dates from file metadata (Exif etc.). It then updates
/// the files' Inode/WinMFT 'Created' and/or 'Modifed' dates accordingly.
/// It requires a directory path as the single argument.
pub fn fix_dates(dir_path: &Path) -> Report {

    let mut report = Report::default();
    let parser = &mut MediaParser::new();

    // Recursively search the directory, filter out any Unix hidden files
    for entry in WalkDir::new(dir_path).into_iter().filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(entry) => {
                let relative_path = get_relative_path(dir_path, &entry);
                // Check OS file metadata
                let metadata = match entry.metadata() {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        report.failed += 1;
                        report.err_msgs.push(format!("{} in '{}'", e, &relative_path));
                        continue;
                     }
                 };

                // Ignore directories but look at all the files
                 if metadata.is_dir() {
                     continue;
                 }
                 report.examined += 1;

                 // Skip probable non-media or unsupported file types by checking the file
                 // extension first in order to avoid unnecessary parsing work
                 if !is_supported_media_file(entry.path()) {
                    report.ignored +=1;
                 }
                 else {
                     // These look like supported media files, so try to parse for image/video metadata
                     match update_file(entry.path(), parser) {
                         Ok(_) => report.updated +=1,
                         Err(e) => {
                             report.failed += 1;
                             report.err_msgs.push(format!("{} in '{}'", e, relative_path));
                         }
                     }
                 }
             },
             // walkdir OS errors (e.g. the path does not exist)
             Err(e) => {
                 report.failed += 1;
                 report.err_msgs.push(e.to_string());
             }
         }
     }
     report
 }

/// Check if the file extension indicates a supported media file
/// as per https://github.com/mindeng/nom-exif
fn is_supported_media_file(file_path: &Path) -> bool {
    if let Some(ext) = file_path.extension() {
        if let Some(ext_str) = ext.to_str() {
            let ext_lower = ext_str.to_lowercase();
            return matches!(
                ext_lower.as_str(),
                | "heic" | "heif" | "jpg" | "jpeg"
                | "tiff" | "tif"  | "iiq" | "raf"
                | "mp4"  | "mov"  | "3gp" | "webm"
                | "mkv"  | "mka"
            );
        }
    }
    false
}

/// Parses a file to determine if it contains suitable image or video metadata
/// then uses the found metadata to update the OS file dates(s)
fn update_file(file_path: &Path, parser: &mut MediaParser) ->  std::result::Result<(), DateFixError> {

    let mut datetimes = DateTimes::default();
    let ms = MediaSource::file_path(file_path)?;

    if ms.has_exif() {
        // .heic, .heif, jpg/jpeg, *.tiff/tif, *.RAF (Fujifilm RAW)
        let iter: ExifIter = parser.parse(ms)?;
        let exif: Exif = iter.into();
        datetimes.created_date = get_image_date(ExifTag::CreateDate, &exif);
        datetimes.modified_date = get_image_date(ExifTag::ModifyDate, &exif);
    }
    else if ms.has_track() {
        // Similar process for video files, but only the Created date is available.
        // ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp
        // or Matroska-based file format: .webm, *.mkv, *.mka
        let info: TrackInfo = parser.parse(ms)?;
        datetimes.created_date = get_video_date(TrackInfoTag::CreateDate, &info);
    }
    else {
        // No metadata of any sort could be found
        return Err(DateFixError::MissingMetadata);
    }

    // Got metadata of some sort, but no dates in it
    if datetimes.created_date.is_none() && datetimes.modified_date.is_none() {
        return Err(DateFixError::MissingDates);
    }

    // We should now have one or both dates, so use the found dates to amemd the file's OS dates
    let file_to_amend = File::options().write(true).open(file_path)?;

    // Changing Created dates requires OS-specific code for Mac & Windows, and cannot be changed at
    // all on Unix-like systems
    cfg_if::cfg_if! {
        if #[cfg(target_os="macos")] {
            use std::os::macos::fs::FileTimesExt;
            if let Some(created) = datetimes.created_date {
                file_to_amend.set_times(FileTimes::new().set_created(SystemTime::from(created)))?;
            }
        }
     else if #[cfg(target_os="windows")] {
            use std::os::windows::fs::FileTimesExt;
            if let Some(created) = datetimes.created_date {
                file_to_amend.set_times(FileTimes::new().set_created(SystemTime::from(created)))?;
            }
        }
        else {
            // Other systems don't have editable 'Created' dates. So, given that we can only
            // obtain 'Created' dates for video files, we will insert the metadata 'Created' date
            // into the 'Modified' date just for these systems. Not ideal, but better
            // than having no original camera dates at all
            if datetimes.created_date.is_some() && !datetimes.modified_date.is_some(){
                datetimes.modified_date = datetimes.created_date;
            }
        }
    }

    // All systems support changing the 'Modified' date
    if let Some(modified) = datetimes.modified_date {
        file_to_amend.set_times(FileTimes::new().set_modified(SystemTime::from(modified)))?;
    }

    Ok(())
}

/// Skip hidden files and directories on Unix-like systems
/// from https://docs.rs/walkdir/latest/walkdir/
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Try to get the file path relative to the top level directory.
/// Defaults to the full file path on failure.
fn get_relative_path(dir_path: &Path, entry: &DirEntry) -> String {
    let full_file_path = entry.path().to_path_buf();
    match diff_paths(&full_file_path, dir_path) {
        Some(short_path) => short_path.display().to_string(),
        None => full_file_path.display().to_string()
    }
}

/// Try to convert an image Exif tag to a DateTime
fn get_image_date(tag: ExifTag, exif: &Exif) -> Option<DateTime<FixedOffset>> {
    if let Some(tag) = exif.get(tag) {
        if let Some(dt) = tag.as_time() {
            return Some(dt);
        }
    }
    None
}

/// Try to convert a video metadata tag to a DateTime
fn get_video_date(tag: TrackInfoTag, info: &TrackInfo ) -> Option<DateTime<FixedOffset>> {
    if let Some(tag) = info.get(tag) {
        if let Some(dt) = tag.as_time() {
            return Some(dt);
        }
    }
    None
}
