use std::fs:: {File, FileTimes};
use std::os::macos::fs::FileTimesExt;
use std::time:: SystemTime;
use std::path::{ Path, PathBuf };
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use chrono::{ DateTime, FixedOffset };
use pathdiff:: diff_paths;

pub struct Report {
    pub files_examined: i32,
    pub files_updated: i32,
    pub files_with_errors: i32,
    pub errors: Vec<String>
}

struct TimeStamps {
    created: Option<DateTime<FixedOffset>>,
    modified: Option<DateTime<FixedOffset>>
}

/// Attempts to fix various media file dates by reading dates from file metadata
/// (EXIF etc.) and updating the file Inode/WinMFT 'Created' and 'Modifed' dates.
pub fn fix_dates<'a>(dir_path: &str, report: &'a mut Report) -> &'a Report {

    if !Path::new(dir_path).exists() {
        report.errors.push(format!("path {} does not exist", dir_path));
        return report;
    }

    let parser = &mut MediaParser::new();

    // Recusively search the directory
    // - filter out any hidden files
    // - ignore any error conditions (e.g. insufficient perms)
    for entry in WalkDir::new(dir_path).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {

        // Get the file path relative to the parent dir to shorten error messages
        let rel_path_buf = get_relative_path(dir_path, &entry);
        let rel_path = rel_path_buf.display().to_string();

        let mut error_found = false;
        match entry.metadata() {
            Ok(metadata) => {
                // Ignore directories and symlinks
                if metadata.is_dir() || metadata.is_symlink() {
                    report.files_examined -= 1
                }
                else if metadata.is_file() {
                    match parse_file(entry.path(), &rel_path.as_str(), parser){
                        Ok(_) => report.files_updated +=1,
                        Err(e) => {
                            // Unsupported file type or missing metadata
                            error_found = true;
                            report.errors.push(format!("{} in {}", e, rel_path));
                        }
                    }
                }
            },
            Err(e) => {
                error_found = true;
                report.errors.push(format!("{} in {}", e, rel_path))
            }
        }
        if error_found {
            report.files_with_errors += 1;
        }
        report.files_examined += 1;
    }
    report
}

/// Parses a file to determine if it has suitable metadata (EXIF etc.)
fn parse_file<'a>(file_path: &Path, rel_path: &str, parser: &mut MediaParser) -> Result<()> {

    let mut timestamps = TimeStamps {
        created: None,
        modified: None
    };

    let ms = MediaSource::file_path(file_path)?;
    if ms.has_exif() {
        // JPG or other image files with EXIF data
        let iter: ExifIter = parser.parse(ms)?;
        let exif: Exif = iter.into();

        // Try to get created date from EXIF data
        let entry_val = exif.get(ExifTag::CreateDate)
            .ok_or_else(|| format!("No Create date in EXIF data in {}", rel_path))?;
        let datetime = entry_val.as_time()
            .ok_or_else(||format!("Could not convert Create tag to datetime in {}", rel_path))?;
        timestamps.created = Some(datetime);

        // Try to get modified date from EXIF data
        let entry_val = exif.get(ExifTag::ModifyDate)
            .ok_or_else(|| format!("No Modify date in EXIF data in {}", rel_path))?;
        let datetime = entry_val.as_time()
            .ok_or_else(|| format!("Could not convert Modify tag to datetime in {}", rel_path))?;
        timestamps.modified = Some(datetime);
    }
    else if ms.has_track() {
       // Video and other files
        let info: TrackInfo = parser.parse(ms)?;
        let trackinfo_tag = info.get(TrackInfoTag::CreateDate)
            .ok_or_else(|| format!("No Create date in EXIF data in {}", rel_path))?;
        let datetime =  trackinfo_tag.as_time()
            .ok_or_else(|| format!("Could not convert Create tag to datetime in {}", rel_path))?;
        timestamps.created = Some(datetime);
    }
    else {
        return Err(format!("No metadata found in {}", rel_path).into());
    }

    // Try to update the file with the found timestamp(s)
    update_file(timestamps, file_path)
}

/// Updates the Created and Modified dates for a file
fn update_file(timestamps: TimeStamps, file_path: &Path) -> Result<()> {

    let file_to_amend = File::options().write(true).open(file_path)?;

    if let Some(created_date) = timestamps.created {
        let new_create_date = FileTimes::new().set_created(SystemTime::from(created_date));
        file_to_amend.set_times(new_create_date)?;
    }
    if let Some(modified_date) = timestamps.modified {
        let new_mod_date = FileTimes::new().set_modified(SystemTime::from(modified_date));
        file_to_amend.set_times(new_mod_date)?;
    }
    Ok(())
}

/// Skip hidden files and directories on Unix-like systems
/// (from https://docs.rs/walkdir/latest/walkdir/)
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Try to get a WalkDir entry file path relative to the parent directory.
/// Defaults to the full WalkDir file path on (very unlikely) failure.
fn get_relative_path(dir_path: &str, entry: &DirEntry) -> PathBuf {
    let full_file_path = entry.path().to_path_buf();
    match diff_paths(&full_file_path, PathBuf::from(dir_path)) {
        Some(short_path) => short_path,
        None => full_file_path
    }
}
