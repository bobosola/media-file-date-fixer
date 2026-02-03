use std::fs::{File, FileTimes};
use std::fmt;
use std::time::SystemTime;
use std::path::Path;
use std::error::Error;
use walkdir::{WalkDir, DirEntry};
use nom_exif::*;
use pathdiff::diff_paths;
use chrono::{DateTime, FixedOffset, Local};

/// Summary report of application run
pub struct Report {
    pub examined: i32,
    pub updated: i32,
    pub failed: i32,
    pub err_msgs: Vec<String>,
    pub time_taken: String
}
impl Default for Report {
    fn default() -> Self {
        Report {
            examined: 0,
            updated: 0,
            failed: 0,
            err_msgs: vec![],
            time_taken: "0".to_string()
        }
    }
}

// Holds any datetimes retrieved from metadata
struct DateTimes {
    created_date: Option<DateTime<FixedOffset>>,
}
impl Default for DateTimes {
    fn default() -> Self {
        DateTimes { created_date: None }
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
/// the files' Inode/WinMFT 'Created' dates (on Mac and Windows) 
/// or the files' 'Modified' dates (on Linux & other Unix-like) accordingly.
/// It requires a directory path as the single argument.
pub fn fix_dates(dir_path: &Path) -> Report {

    let mut report = Report::default();
    let parser = &mut MediaParser::new();
    let start = std::time::Instant::now();

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
     report.time_taken = format!("Completed in {:.2}s", start.elapsed().as_secs_f64());
     report
 }
 
 
/// Extracts a datetime from media metadata (EXIF or video track info).
/// 
/// Takes a metadata source and a tag, retrieves the raw entry value,
/// then converts it to a timezone-aware datetime using get_wall_clock_date_time.
/// Returns None if the tag is missing or the date cannot be parsed.
macro_rules! get_date {
    ($source:expr, $tag:expr) => {
        $source.get($tag).and_then(get_wall_clock_date_time)
    };
}

/// Parses a file to determine if it contains suitable image or video metadata
/// then uses the found metadata to update the OS file dates(s)
fn update_file(file_path: &Path, parser: &mut MediaParser) -> std::result::Result<(), DateFixError> {

    let mut datetimes = DateTimes::default();
    let ms = MediaSource::file_path(file_path)?;

    if ms.has_exif() {
        // .heic, .heif, jpg/jpeg, *.tiff/tif, *.RAF (Fujifilm RAW)
        let iter: ExifIter = parser.parse(ms)?;
        let exif: Exif = iter.into();
        datetimes.created_date = get_date!(&exif, ExifTag::CreateDate);
    }
    else if ms.has_track() {
        // Similar process for video files
        // ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp
        // or Matroska-based file format: .webm, *.mkv, *.mka
        let info: TrackInfo = parser.parse(ms)?;
        datetimes.created_date = get_date!(&info, TrackInfoTag::CreateDate);
    }
    else {
        // No metadata of any sort could be found
        return Err(DateFixError::MissingMetadata)
    }

    // Got metadata of some sort, but no created dates in it
    if datetimes.created_date.is_none() {
        return Err(DateFixError::MissingDates)
    }

    // Use the found created date to amend the file's OS dates
    let file_to_amend = File::options().write(true).open(file_path)?;

    // Changing Created dates requires OS-specific code for Mac & Windows, and cannot be changed at
    // all on Unix-like systems, so edit the Modified date in such cases
    if let Some(created) = datetimes.created_date {
        cfg_if::cfg_if! {
            if #[cfg(target_os="macos")] {
                use std::os::macos::fs::FileTimesExt;
                file_to_amend.set_times(FileTimes::new().set_created(SystemTime::from(created)))?;
            }
            else if #[cfg(target_os="windows")] {
                use std::os::windows::fs::FileTimesExt;
                file_to_amend.set_times(FileTimes::new().set_created(SystemTime::from(created)))?;
            }
            else {
                // Other systems don't have editable 'Created' dates. So, we will insert the 
                // metadata 'Created' date into the 'Modified' date just for these systems. 
                // Not ideal, but better than having no original camera dates at all
                file_to_amend.set_times(FileTimes::new().set_modified(SystemTime::from(created)))?;
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
    let full_file_path = entry.path();
    match diff_paths(&full_file_path, dir_path) {
        Some(short_path) => short_path.display().to_string(),
        None => full_file_path.display().to_string()
    }
}

/// Get a local 'wall clock' DateTime from the media metadata
/// and convert it to a system DateTime showing the same local time
/// i.e. it ignores any time zone information so that a media file
/// created at 15:00 in New York still appears as 15:00 when inserted
/// into a file copy made in any other location or time zone
fn get_wall_clock_date_time(entryval: &EntryValue) -> Option<DateTime<FixedOffset>> {   
    if let Some(naive_with_maybe_offset) = entryval.as_time_components() {
        let (naive, _) = naive_with_maybe_offset;
        if let Some(local_datetime) = naive.and_local_timezone(Local).latest() {
            return Some(local_datetime.into())
        }  
    }
    None
}
