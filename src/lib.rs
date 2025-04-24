use std::fs:: {File, FileTimes};
use std::os::macos::fs::FileTimesExt;
use std::time:: SystemTime;
use std::path::{ Path, PathBuf };
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use chrono::{ DateTime, FixedOffset };
use pathdiff:: diff_paths;
use formatx::formatx;

pub struct Report {
    pub examined: i32,
    pub updated: i32,
    pub failed: i32,
    pub errors: Vec<String>
}
impl Default for Report {
    fn default() -> Self {
        return Report {
            examined: 0,
            updated: 0,
            failed: 0,
            errors: vec![]
        }
    }
}

struct TimeStamps {
    created: Option<DateTime<FixedOffset>>,
    modified: Option<DateTime<FixedOffset>>
}
impl Default for TimeStamps {
    fn default() -> Self {
        return TimeStamps {
            created: None,
            modified: None
        }
    }
}

struct ErrorMsg {
    no_create: String,
    no_modify: String,
    bad_create: String,
    bad_modify: String,
    no_metadata: String,
    with_path: String,
    no_path: String,
}
impl Default for ErrorMsg {
    fn default() -> Self {
        return ErrorMsg {
            no_create: String::from("No Create date in Exif metadata in {}"),
            no_modify: String::from("No Modify date in Exif metadata in {}"),
            bad_create: String::from("Could not convert Create tag to datetime in {}"),
            bad_modify: String::from("Could not convert Modify tag to datetime in {}"),
            no_metadata: String::from("No media metadata found in {}"),
            with_path: String::from("{} in {}"),
            no_path: String::from("{}")
        }
    }
}

/// Attempts to fix various media file dates by reading dates from file metadata
/// (Exif etc.) and updating the file Inode/WinMFT 'Created' and 'Modifed' dates.
pub fn fix_dates<'a>(dir_path: &str) -> Report {

    let mut report = Report::default();
    let err_msg = ErrorMsg::default();
    let parser = &mut MediaParser::new();

    // Recursively search the directory, filter out any Unix hidden files
    for entry in WalkDir::new(dir_path).into_iter().filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(entry) => {

                // Get the file path relative to the parent dir
                let rel_path = get_relative_path(dir_path, &entry).display().to_string();

                match entry.metadata() {
                    Ok(metadata) => {
                        // Ignore directory entries
                        // (symlinks are ignored by default)
                        if metadata.is_dir() {
                            report.examined -= 1
                        }
                        else if metadata.is_file() {
                            // Check the file for image or video metadata
                            // and try to update the timestamps
                            match parse_file(entry.path(), &rel_path.as_str(), parser){
                                Ok(_) => report.updated +=1,
                                Err(e) => {
                                    // nom_exif errors
                                    report.failed += 1;
                                    report.errors.push(get_msg(formatx!(&err_msg.with_path, e, rel_path)));
                                }
                            }
                        }
                    },
                    Err(e) => {
                        // WalkDir file metadata custom errors for path values that the program does not have
                        // permissions to access or if the path does not exist
                        report.failed += 1;
                        report.errors.push(get_msg(formatx!(&err_msg.with_path, e, rel_path)));
                    }
                }
                report.examined += 1;
            },
            Err(e) => {
               // WalkDir iteration errors
                report.failed += 1;
                report.errors.push(get_msg(formatx!(&err_msg.no_path, e)));
            }
        }
    }
    report
}

/// Parses a file to determine if it has suitable metadata
/// and uses the found metadata to update the file timestamps
fn parse_file<'a>(file_path: &Path, rel_path: &str, parser: &mut MediaParser) -> Result<()> {

    let mut timestamps = TimeStamps::default();
    let err_msg = ErrorMsg::default();
    let ms = MediaSource::file_path(file_path)?;

    if ms.has_exif() {
        // Images files in various formats with Exif data
        let iter: ExifIter = parser.parse(ms)?;
        let exif: Exif = iter.into();

        // Try to get the Created date tag
        let exif_tag = exif.get(ExifTag::CreateDate)
            .ok_or_else(|| get_msg(formatx!(&err_msg.no_create, rel_path)))?;

        // If found, try to convert the tag to a datetime
        let datetime = exif_tag.as_time()
            .ok_or_else(|| get_msg(formatx!(&err_msg.bad_create, rel_path)))?;

        // Store the datetime obtained
        timestamps.created = Some(datetime);

        // Same process for Modified date
        let exif_tag = exif.get(ExifTag::ModifyDate)
            .ok_or_else(|| get_msg(formatx!(&err_msg.no_modify, rel_path)))?;
        let datetime = exif_tag.as_time()
            .ok_or_else(|| get_msg(formatx!(&err_msg.bad_modify, rel_path)))?;
        timestamps.modified = Some(datetime);

    }
    else if ms.has_track() {

        // Video files in various formats
        let info: TrackInfo = parser.parse(ms)?;

        // Same process for video files, but only the Created date
        // is available in TrackInfo
        let track_tag = info.get(TrackInfoTag::CreateDate)
            .ok_or_else(|| get_msg(formatx!(&err_msg.no_create, rel_path)))?;
        let datetime = track_tag.as_time()
            .ok_or_else(|| get_msg(formatx!(&err_msg.bad_create, rel_path)))?;
        timestamps.created = Some(datetime);
    }
    else {
        // Catch-all
        return Err(get_msg(formatx!(&err_msg.no_metadata, rel_path)).into());
    }

    // Try to update the file with the found timestamp(s)
    let file_to_amend = File::options().write(true).open(file_path)?;

    // Update the Created date (if we have one)
    if let Some(created_date) = timestamps.created {
        let new_create_date = FileTimes::new().set_created(SystemTime::from(created_date));
        file_to_amend.set_times(new_create_date)?;
    }
    // Update the Modified date (if we have one)
    if let Some(modified_date) = timestamps.modified {
        let new_mod_date = FileTimes::new().set_modified(SystemTime::from(modified_date));
        file_to_amend.set_times(new_mod_date)?;
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
fn get_relative_path(dir_path: &str, entry: &DirEntry) -> PathBuf {
    let full_file_path = entry.path().to_path_buf();
    match diff_paths(&full_file_path, PathBuf::from(dir_path)) {
        Some(short_path) => short_path,
        None => full_file_path
    }
}

/// Utility to avoid panics using formatx!().unwrap()
fn get_msg(res: std::result::Result<String, formatx::Error>) -> String {
    match res {
        Ok(msg) => msg,
        Err(e) => format!("Formatx error {}", e )
    }
}
