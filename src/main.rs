use std::fs:: {File, FileTimes};
use std::time:: SystemTime;
use walkdir::{ WalkDir, DirEntry };
use nom_exif::*;
use std::path::Path;
use chrono::{ DateTime, FixedOffset };

fn main() -> () {
    let dir_path = "/Users/bobosola/Movies/test_videos/test1";
    match fixdates(dir_path) {
        Ok(filecounter) => println!("Finished. Processed {:?} files.", filecounter),
        Err(e) => eprintln!("{:?}", e)
    };
}

fn fixdates(dir_path: &str) -> Result<i32> {

    let mut filecounter = 0;
    let mut parser = MediaParser::new();

    // Filter out any hidden or or error condition files and directories (e.g. insufficient perms)
    for entry in WalkDir::new(dir_path).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {

        // Only process file types
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                filecounter += 1;
                let file_path = entry.path();

                // Try to get media data from the file
                let ms = MediaSource::file_path(file_path)?;

                if ms.has_exif() {
                    let iter: ExifIter = parser.parse(ms)?;
                    let exif: Exif = iter.into();
                    if let Some(entry_val) = exif.get(ExifTag::CreateDate) {
                        match entry_val.as_time() {
                            Some(datetime) => {
                                println!("CreateDate tag {:?} for {:?}", datetime, file_path);
                                match update_file(datetime, file_path){
                                    Ok(_) => (),
                                    Err(e) => eprintln!("{:?}", e)
                                }
                            },
                            None => eprintln!("No CreateDate tag found in EXIF data for {:?}", file_path)
                         }
                    }
                    else {
                        println!("Could not get entry value for {}", entry.path().display());
                    }
                } else if ms.has_track() {

                    let info: TrackInfo = parser.parse(ms)?;
                    if let Some(datetime) = info.get(TrackInfoTag::CreateDate) {
                        match datetime.as_time() {
                            Some(datetime) => {
                                println!("CreateDate info {:?} for {:?}", datetime, file_path);
                                match update_file(datetime, file_path){
                                    Ok(_) => (),
                                    Err(e) => eprintln!("{:?}", e)
                                }
                            },
                            None => eprintln!("No CreateDate info found for {:?}", file_path)
                        }
                    }
                }
            }
        }
        else {
            eprintln!("No file metadata for {}", entry.path().display());
        }
    }
    Ok(filecounter)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn update_file(datetime: DateTime<FixedOffset>, filepath: &Path ) -> Result<()> {

    // Prepare the updated timestamps to be used on the file
    // using the created datetime obtained from the file
    let filetimes = FileTimes::new().set_modified(SystemTime::from(datetime));

    // Change the 'file modified' date to the updated timestamps
    let file_to_amend = File::options().write(true).open(filepath)?;
    file_to_amend.set_times(filetimes)?;
    Ok(())
}
