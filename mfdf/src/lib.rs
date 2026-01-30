use std::fs:: {File, FileTimes };
use std::fmt;
use std::time:: SystemTime;
use std::path::{ Path };
use std::error::Error;
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use pathdiff:: diff_paths;
use chrono:: {DateTime, FixedOffset, NaiveDateTime, TimeZone};

/// Summary report of application run
pub struct Report {
    pub examined: i32,
    pub updated: i32,
    pub failed: i32,
    pub err_msgs: Vec<String>
}
impl Default for Report {
    fn default() -> Self {
        return Report {
            examined: 0,
            updated: 0,
            failed: 0,
            err_msgs: vec![]
        }
    }
}

// Holds any datetimes retrieved from metadata
struct DateTimes {
    created_date: Option<DateTime<FixedOffset>>,
}
impl Default for DateTimes {
    fn default() -> Self {
        return DateTimes {
            created_date: None,
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

/// Attempts to fix lost Created dates in common media files
/// by recovering the dates from file metadata (Exif etc.). It then updates
/// the files' Inode/WinMFT 'Created' (and/or 'Modifed' dates) accordingly.
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

                match update_file(entry.path(), parser) {
                    Ok(_) => report.updated +=1,
                    Err(e) => {
                        report.failed += 1;
                        report.err_msgs.push(format!("{} in '{}'", e, relative_path));
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
    }
    else if ms.has_track() {
        // Similar process for video files
        // ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp
        // or Matroska-based file format: .webm, *.mkv, *.mka
        let info: TrackInfo = parser.parse(ms)?;
        datetimes.created_date = get_video_date(TrackInfoTag::CreateDate, &info);
    }
    else {
        // No metadata of any sort could be found
        return Err(DateFixError::MissingMetadata);
    }

    // Got metadata of some sort, but no created dates in it
    if datetimes.created_date.is_none() {
        return Err(DateFixError::MissingDates);
    }

    // Use the found created date to amemd the file's OS dates
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
            // Other systems don't have editable 'Created' dates. So, we will insert the 
            // metadata 'Created' date into the 'Modified' date just for these systems. 
            // Not ideal, but better than having no original camera dates at all
            if datetimes.created_date.is_some() {
                datetimes.modified_date = datetimes.created_date;
            }
        }
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


/// Exif datetimes should be in standard format with offset, e.g. '2026-01-16T15:29:19+00:00'
/// but they might also be 'naive' date format e.g. '2026-01-16 15:29:19'
/// as seen in iPhone HEIC images converted to JPG in the iPhone Files app.
/// So we need to try to convert any 'naive' dates found to standard format
fn get_image_date(tag: ExifTag, exif: &Exif) -> Option<DateTime<FixedOffset>> {
    if let Some(tag) = exif.get(tag) {
        if let Some(dt) = tag.as_time() {   
            return Some(dt);
        } else {
            return get_dt_from_naive_dt(&tag);
        }
    }
    None
}

/// Try to convert a video metadata datetime tag to a DateTime
fn get_video_date(tag: TrackInfoTag, info: &TrackInfo ) -> Option<DateTime<FixedOffset>> {
    if let Some(tag) = info.get(tag) {
        if let Some(dt) = tag.as_time() {
            return Some(dt);
        }
    }
    None
}

/// Trys to convert 'naive' datetimes to standard datetimes with offset
fn get_dt_from_naive_dt(entry: &EntryValue) -> Option<DateTime<FixedOffset>> {
    if let Some(naive) = NaiveDateTime::parse_from_str(&entry.to_string(),"%Y-%m-%d %H:%M:%S").ok(){
        let offset = FixedOffset::east_opt(0).unwrap(); // zero is safe to unwrap
        if let Some(dt) = offset.from_local_datetime(&naive).single(){
            return Some(dt);
        }      
    } 
   None 
}
