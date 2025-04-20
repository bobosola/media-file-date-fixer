use std::fs:: {File, FileTimes};
use std::time:: SystemTime;
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use std::path::Path;
use chrono::{ DateTime, FixedOffset };

struct Report {
    files_examined: i32,
    files_updated: i32,
    files_ignored: i32,
    errors: Vec<String>
}

struct Changes {
    update_created: bool,
    update_modified: bool
}

struct TimeStamps {
    created: Option<DateTime<FixedOffset>>,
    modified: Option<DateTime<FixedOffset>>
}

fn main() -> () {

    let dir_path = "/Users/bobosola/Movies/test_videos/test1";
    let changes_to_make = &Changes {
        update_created: true,
        update_modified: true
    };

    if changes_to_make.update_created == false && changes_to_make.update_modified == false {
         println!("No files examined or changed");
    }
    else {
        let report = &mut Report {
            files_examined: 0,
            files_updated: 0,
            files_ignored: 0,
            errors: vec![]
        };
        let results = fixdates(changes_to_make , dir_path, report);

        println!("\nFiles examined: {}", results.files_examined);
        println!("Files updated: {}", results.files_updated);
        println!("Files ignored: {} (unsupported file types)", results.files_ignored);

        let num_errors = results.errors.len();
        if num_errors > 0 {
            println!("Files failed to update due to errors: {}", num_errors);
            const NUM_DASHES: usize = 80;
            let dashes = "-".repeat(NUM_DASHES);
            println!("{}", dashes);
            for str_error_msg in &results.errors {
                println!("Error: {}.", str_error_msg);
                println!("{}", dashes);
            }
        }
    }
}

fn fixdates<'a>(changes_to_make: &Changes, dir_path: &str, report: &'a mut Report) -> &'a Report {

    let parser = &mut MediaParser::new();

    // Filter out any hidden or error condition files and directories (e.g. insufficient perms)
    for entry in WalkDir::new(dir_path).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {

        report.files_examined += 1;
        match entry.metadata() {
            Ok(metadata) => {
                if metadata.is_file() {
                    match parse_file(changes_to_make, entry.path(), parser, report){
                        Ok(_) => report.files_updated +=1 ,
                        Err(_) => {
                            // Unsupported file type
                            report.files_ignored +=1;
                        }
                    }
                }
            },
            Err(e) => report.errors.push(format!("{} in {:?}", e, entry.path().display()))
        }
    }
    report
}

fn parse_file<'a>(changes_to_make: &Changes, file_path: &Path, parser: &mut MediaParser, report: &'a mut Report) -> Result<()> {

    let mut timestamps = TimeStamps {
        created: None,
        modified: None
    };
    let ms = MediaSource::file_path(file_path)?;

    // JPG or other image files with EXIF data
    if ms.has_exif() {

        let iter: ExifIter = parser.parse(ms)?;
        let exif: Exif = iter.into();

        // Try to get created date
        if changes_to_make.update_created {
            if let Some(entry_val) = exif.get(ExifTag::CreateDate) {
                match entry_val.as_time() {
                    Some(datetime) => {
                        timestamps.created = Some(datetime);
                    },
                    None => report.errors.push(format!("Could not convert CreateDate to datetime in {:?}", file_path.display()))
                }
            } else {
                report.errors.push(format!("No CreateDate in EXIF data in {:?}", file_path))
            }
        }

        // Try to get modified date
        if changes_to_make.update_modified {
            if let Some(entry_val) = exif.get(ExifTag::ModifyDate) {
                match entry_val.as_time() {
                    Some(datetime) => {
                        timestamps.modified = Some(datetime);
                    },
                    None => report.errors.push(format!("Could not convert ModifyDate to datetime in {:?}", file_path.display()))
                }
            } else {
                report.errors.push(format!("No ModifyDate in EXIF data in {:?}", file_path))
            }
        }

    // Video and other files
    } else if ms.has_track() {

        let info: TrackInfo = parser.parse(ms)?;

        // NB: no ModifyDateis available for TrackInfo type so use CreateDate value for both
        if let Some(datetime) = info.get(TrackInfoTag::CreateDate) {
            match datetime.as_time() {
                Some(datetime) => {
                    if changes_to_make.update_created {
                        timestamps.created = Some(datetime);
                    }
                    if changes_to_make.update_modified {
                        timestamps.modified = Some(datetime);
                    }
                },
                None => report.errors.push(format!("No CreateDate found in {:?}", file_path))
            }
        }
    }
    else {
        report.errors.push(format!("No dates found in {:?}", file_path));
        return Ok(())
    }

    // Got one or both dates to update
    match update_file(timestamps, file_path){
        Ok(_) => (),
        Err(e) => report.errors.push(format!("{:?} in {:?}", e, file_path.display()))
    }
    Ok(())
}

fn update_file(timestamps: TimeStamps, filepath: &Path) -> Result<()> {

    let file_to_amend = File::options().write(true).open(filepath)?;

    if let Some(created_date) = timestamps.created {
        let new_create_date = FileTimes::new().set_modified(SystemTime::from(created_date));
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
