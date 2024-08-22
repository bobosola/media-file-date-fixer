use std::fs::File;
use std::io::BufReader;
use mp4::creation_time;
use std::time::{ UNIX_EPOCH, Duration };
use std::fs::FileTimes;
use walkdir::{DirEntry, WalkDir};
use anyhow::Result;

fn main() -> Result<()> {

    let dir_path = "/home/bob/Videos/test1";

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {

        if !is_mp4(&entry) {
            continue;
        }

        let path = entry.path();

        // Get MP4 metadata
        let file = File::open(path)?;
        let filesize = file.metadata()?.len();
        let reader = BufReader::new(file);
        let mp4 = mp4::Mp4Reader::read_header(reader, filesize)?;

        // Get MP4 creation datetime (a Unix timestamp) from metadata
        // and convert to SystemTime
        let ts = creation_time(mp4.moov.mvhd.creation_time);
        let created_date = UNIX_EPOCH + Duration::from_secs(ts);

        // Prepare the date to be used on a file
        let filetimes = FileTimes::new().set_modified(created_date);

        // Change the 'file modified' date to the metadata creation date
        let file_to_amend = File::options().write(true).open(path)?;
        file_to_amend.set_times(filetimes)?;
        println!("Converted: {}", path.display());
    }

    fn is_mp4(entry: &DirEntry) -> bool {

        let filename = match entry.file_name().to_str() {
            Some(name) => name,
            None => return false
        };

        if entry.file_type().is_file() && 
           filename.to_lowercase().ends_with("mp4") && 
           !filename.starts_with(".") {
            return true
        }
        false
    }

    Ok(())
}