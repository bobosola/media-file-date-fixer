use std::fs:: {File, FileTimes};
use std::os::macos::fs::FileTimesExt;
use std::str::FromStr;
use std::time:: SystemTime;
use std::path::{ Path, PathBuf };
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use chrono::{ DateTime, FixedOffset };
use pathdiff:: diff_paths;

/// Holds output data
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

pub fn fix_dates<'a>(dir_path: &str, report: &'a mut Report) -> &'a Report {

    if !Path::new(dir_path).exists() {
        report.errors.push(format!("Path {:?} does not exist", dir_path));
        return report
    }

    let parser = &mut MediaParser::new();

    // Recusively search the directory
    // - filter out any hidden files
    // - ignore any error conditions (e.g. insufficient perms)
    for entry in WalkDir::new(dir_path).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {

        report.files_examined += 1;

        // Try to get the relative file path from the dir path to reduce length of error messages
        // Defaults to full length dir path on failure
        let short_path = match diff_paths(entry.path().as_os_str().to_str().unwrap_or(dir_path), dir_path){
            Some(pathbuf) => pathbuf,
            None => PathBuf::from_str(dir_path).unwrap()
        };
        let rel_path = short_path.as_os_str().to_str().unwrap_or(dir_path);

        match entry.metadata() {
            Ok(metadata) => {
                if metadata.is_dir() || metadata.is_symlink() {
                    report.files_examined -= 1
                }
                else if metadata.is_file() {
                    match parse_file(entry.path(), parser){
                        Ok(_) => report.files_updated +=1,
                        Err(e) => {
                            // Unsupported file type or missing metadata
                            report.files_with_errors += 1;
                            report.errors.push(format!("{} in {:?}", e, rel_path));
                        }
                    }
                }
            },
            Err(e) => {
                report.files_with_errors += 1;
                report.errors.push(format!("{} in {:?}", e, rel_path))
            }
        }
    }
    report
}

fn parse_file<'a>(file_path: &Path, parser: &mut MediaParser) -> Result<()> {

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
        if let Some(entry_val) = exif.get(ExifTag::CreateDate) {
            match entry_val.as_time() {
                Some(datetime) => timestamps.created = Some(datetime),
                None => return Err(format!("Could not convert Create date to datetime in {:?}", file_path.display()).into())
            }
        } else {
            return Err(format!("No Create date in EXIF data in {:?}", file_path.display()).into())
        }

        // Try to get modified date from EXIF data
        if let Some(entry_val) = exif.get(ExifTag::ModifyDate) {
            match entry_val.as_time() {
                Some(datetime) => timestamps.modified = Some(datetime),
                None => return Err(format!("Could not convert Modify date to datetime in {:?}", file_path.display()).into())
            }
        } else {
            return Err(format!("No Modify date in EXIF data in {:?}", file_path.display()).into())
        }

    }
    else if ms.has_track() {
       // Video and other files
        let info: TrackInfo = parser.parse(ms)?;

        // NB: only CreateDate is available in TrackInfo type
        if let Some(datetime) = info.get(TrackInfoTag::CreateDate) {
            match datetime.as_time() {
                Some(datetime) => timestamps.created = Some(datetime),
                None => return Err(format!("Could not convert Create date to datetime in {:?}", file_path.display()).into())
            }
        }
        else {
            return Err(format!("No Create date in EXIF data in {:?}", file_path.display()).into())
        }
    }
    else {
        return Err(format!("No metadata found in {:?}", file_path.display()).into())
    }

    // Try to update the file with the found timestamp(s)
    match update_file(timestamps, file_path){
        Ok(_) => (),
        Err(e) => return Err(format!("{:?} in {:?}", e, file_path.display()).into())
    }
    Ok(())
}

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

// From https://docs.rs/walkdir/latest/walkdir/
// Skip hidden files and directories on unix
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
