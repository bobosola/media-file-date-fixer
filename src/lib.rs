use std::fs:: {File, FileTimes };
#[cfg(target_os = "macos")]
use std::os::macos::fs::FileTimesExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::FileTimesExt;
use std::time:: SystemTime;
use std::path::{ Path, PathBuf };
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use pathdiff:: diff_paths;
use std::error::Error;

use crate::types::types::*;
mod types;

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
                            match parse_file(entry.path(), &rel_path, parser){
                                Ok(_) => report.updated +=1,
                                Err(e) => {
                                    // nom_exif errors
                                    report.failed += 1;
                                    report.errors.push(format!("{} in '{}'", e, &rel_path));
                                }
                            }
                        }
                    },
                    Err(e) => {
                        // WalkDir OS errors where the program does not have access perms
                        // or if the path does not exist (with path shortened to relative path)
                        report.failed += 1;
                        report.errors.push(format!("{} in '{}'", e, &rel_path).replace(dir_path, ""));
                    }
                }
                report.examined += 1;
            },
            Err(e) => {
               // Top level WalkDir OS error (e.g. no perms to argument dir_path)
                report.failed += 1;
                report.errors.push(e.to_string());
            }
        }
    }
    report
}

/// Parses a file to determine if it has suitable metadata then uses
/// the found data to update the file dates(s)
fn parse_file(file_path: &Path, rel_path: &str, parser: &mut MediaParser) -> std::result::Result<(), Box<dyn Error>> {

    let ms = MediaSource::file_path(file_path)?;
    let mut datetimes = DateTimes::default();

    if ms.has_exif() {
        // .heic, .heif, jpg/jpeg, *.tiff/tif, *.RAF (Fujifilm RAW)

        let iter: ExifIter = parser.parse(ms)?;
        let exif: Exif = iter.into();

        // If a 'Created' date tag is found, try to convert it to a DateTime
        let exif_tag = exif.get(ExifTag::CreateDate)
           .ok_or_else(|| MissingCreateDateError{file_path: rel_path.into()})?;
        let dt = exif_tag.as_time()
            .ok_or_else(|| BadCreateDateError{file_path: rel_path.into()})?;
        datetimes.created_date = Some(dt);

        // Same for 'Modified' date
        // Note that if the 'Created' date cannot be obtained, then the function
        // exits without trying to find a 'Modified' date. That's because if
        // there's no 'Creation' date, then it's a very safe bet that there won't
        // be a 'Modifed date' either.
        let exif_tag = exif.get(ExifTag::ModifyDate)
           .ok_or_else(|| MissingModifyDateError{file_path: rel_path.into()})?;
        let dt = exif_tag.as_time()
            .ok_or_else(|| BadModifyDateError{file_path: rel_path.into()})?;
        datetimes.modified_date = Some(dt);
    }
    else if ms.has_track() {
        // ISO base media file format (ISOBMFF): *.mp4, *.mov, *.3gp
        // or Matroska-based file format: .webm, *.mkv, *.mka

        let info: TrackInfo = parser.parse(ms)?;

        // Same process for video files, but only the Created data is available
        let track_tag = info.get(TrackInfoTag::CreateDate)
            .ok_or_else(|| MissingCreateDateError{ file_path: rel_path.into() })?;
        let dt = track_tag.as_time()
            .ok_or_else(|| BadCreateDateError{ file_path: rel_path.into() })?;
        datetimes.created_date = Some(dt);
    }

    // Update the file if we have retrieved any valid datetimes
    if let Some(created) = datetimes.created_date {
        let file_to_amend = File::options().write(true).open(file_path)?;
        file_to_amend.set_times(FileTimes::new().set_created(SystemTime::from(created)))?;
        if let Some(modified) = datetimes.modified_date {
            file_to_amend.set_times(FileTimes::new().set_modified(SystemTime::from(modified)))?;
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
fn get_relative_path(dir_path: &str, entry: &DirEntry) -> String {
    let full_file_path = entry.path().to_path_buf();
    match diff_paths(&full_file_path, PathBuf::from(dir_path)) {
        Some(short_path) => short_path.display().to_string(),
        None => full_file_path.display().to_string()
    }
}
