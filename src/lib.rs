use std::fs:: {File, FileTimes };
use std::fmt;
use std::time:: SystemTime;
use std::path::{ Path, PathBuf };
use std::error::Error;
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use pathdiff:: diff_paths;
use chrono:: {DateTime, FixedOffset };

// Summary report of application run
pub struct Report {
    pub examined: i32,
    pub updated: i32,
    pub errors: i32,
    pub err_msgs: Vec<String>
}
impl Default for Report {
    fn default() -> Self {
        return Report {
            examined: 0,
            updated: 0,
            errors: 0,
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

// Error for failure to read or parse both datetimes
#[derive(Debug)]
struct MissingDatesError {}
impl fmt::Display for MissingDatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No dates found in metadata")
    }
}
impl Error for MissingDatesError {}

/// Attempts to fix various media file dates by reading dates from file metadata
/// (Exif etc.) and updating the file Inode/WinMFT 'Created' and 'Modifed' dates.
pub fn fix_dates(dir_path: &str) -> Report {

    let mut report = Report::default();
    let parser = &mut MediaParser::new();

    // Recursively search the directory, filter out any Unix hidden files and
    // only consider files for parsing
    for entry in WalkDir::new(dir_path).into_iter().filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(entry) => {
                let rel_path = get_relative_path(dir_path, &entry);
                match entry.metadata() {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            report.examined -= 1
                        }
                        else if metadata.is_file() {
                            match update_file(entry.path(), parser){
                                Ok(_) => report.updated +=1,
                                Err(e) => {
                                    // nom_exif and MissingDate errors
                                    report.errors += 1;
                                    report.err_msgs.push(format!("{} in '{}'", e, &rel_path));
                                }
                            }
                        }
                    },
                    Err(e) => {
                        // walkdir OS errors where the program does not have access perms
                        // or if the path does not exist (with path shortened to relative path)
                        report.errors += 1;
                        report.err_msgs.push(format!("{}", e).replace(dir_path, ""));
                    }
                }
                report.examined += 1;
            },
            Err(e) => {
               // Top level walkdir OS error (e.g. no perms to argument directory)
                report.errors += 1;
                report.err_msgs.push(e.to_string());
            }
        }
    }
    report
}

/// Parses a file to determine if it has suitable metadata
/// then uses the found data to update the file dates(s)
fn update_file(file_path: &Path, parser: &mut MediaParser) -> std::result::Result<(), Box<dyn Error>> {

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
        // ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp
        // or Matroska-based file format: .webm, *.mkv, *.mka
        // Similar process for video files, but only the Created date is available
        let info: TrackInfo = parser.parse(ms)?;
        datetimes.created_date = get_video_date(TrackInfoTag::CreateDate, &info);
    }

    // Update the file if we have retrieved any valid datetimes
    if datetimes.created_date.is_some() || datetimes.modified_date.is_some() {
        let file_to_amend = File::options().write(true).open(file_path)?;

        // 'Created' dates on Unix-like systems (other than MacOS) are a big nasty mess. They either have
        // none at all, or they might have non-POSIX extensions which record a 'Created' date, but under
        // differing names under differing systems, none of which can be changed via an API.
        // See https://www.figuiere.net/technotes/notes/tn005/. But we can alter the 'Modified' date.

        cfg_if::cfg_if! {
            if #[cfg(target_os="macos")] {
                // MacOS supports changing the 'Created' date
                use std::os::macos::fs::FileTimesExt;
                if let Some(created) = datetimes.created_date {
                    file_to_amend.set_times(FileTimes::new().set_created(SystemTime::from(created)))?;
                }
            }
            else if #[cfg(target_os="windows")] {
                // Windows supports changing the 'Created' date
                #[cfg(target_os = "windows")]
                use std::os::windows::fs::FileTimesExt;
                if let Some(created) = datetimes.created_date {
                    file_to_amend.set_times(FileTimes::new().set_created(SystemTime::from(created)))?;
                }
            }
            else {
                // Other Unix-like systems don't have editable 'Created' dates. So, given that we cannot
                // obtain 'Modified' dates for video files, we will insert the metadata 'Created' date
                // into the 'Modified' date for these systems. Not ideal, but better
                // than having no original camera dates at all!
                if datetimes.created_date.is_some() && !datetimes.modified_date.is_some(){
                    datetimes.modified_date = datetimes.created_date;
                }
            }
        }
        // All systems support changing the 'Modified' date.
        if let Some(modified) = datetimes.modified_date {
            file_to_amend.set_times(FileTimes::new().set_modified(SystemTime::from(modified)))?;
        }
    }
    else {
        Err(MissingDatesError{})?;
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
fn get_relative_path(dir_path: &str, entry: &DirEntry) -> String {
    let full_file_path = entry.path().to_path_buf();
    match diff_paths(&full_file_path, PathBuf::from(dir_path)) {
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
