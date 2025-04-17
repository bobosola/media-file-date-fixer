use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::fs::FileTimes;
use walkdir::{DirEntry, WalkDir};
use anyhow::{ anyhow, Context, Result };
use file_format::FileFormat;
use nom_exif::*;
use nom_exif::ExifTag::*;
use chrono::{ DateTime, Tz };

fn main() -> Result<()> {

    let dir_path = "/home/bob/Videos/test1";

    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {

        let file_path = entry.path();
        let file_format = FileFormat::from_file(file_path).context("Could not determine file format")?;
        match parse_file(file_format, file_path) {
            Ok(systime) => amend_file_date(systime, file_path),
            _ => ()
        };

    }
    Ok(())
}

    fn parse_file(file_format: FileFormat, file_path: &Path) -> Result<SystemTime> {

        match file_format {
            // .jpg image file
            FileFormat::JointPhotographicExpertsGroup => {
                let f = File::open(file_path)?;
                match parse_jpeg_exif(f)? {
                    Some(exif) => {
                        return get_creation_date_from_exif(exif)
                    },
                    _ => ()
                }            
            },
            // .heic or .heif image file
            FileFormat::HighEfficiencyImageCoding | FileFormat::HighEfficiencyImageFileFormat=> {
                let f = File::open(file_path)?;
                match parse_heif_exif(f)? {
                    Some(exif) => {
                        return get_creation_date_from_exif(exif)
                    },
                    _ => ()
                }            
            },            
            _ => ()
        }
        Err(anyhow!("File format {:?}", file_path))
    }

    fn get_exif (fmt: FileFormat, file_path: &Path) -> Result<Option<Exif>> {

        let f = File::open(file_path).with_context(|| format!("Could not open file {:?}", file_path))?;

        let exif = match fmt {
            FileFormat::JointPhotographicExpertsGroup => {
                match parse_jpeg_exif(f) {
                    Ok(ex) => ex,
                    _ => None
                } 
            },
            FileFormat::HighEfficiencyImageCoding | FileFormat::HighEfficiencyImageFileFormat => {
                match parse_heif_exif(f) {
                    Ok(ex) => ex,
                    _ => None
                } 
            },
           _ => None
        };
        Ok(exif)
    }

    fn get_creation_date_from_exif(exif: Exif) -> Result<SystemTime> {

        match exif.get_value(&CreateDate)? {
            Some(val) => {
                let dt = DateTime::parse_from_rfc3339(&val.to_string()).context("Non-standard EXIF date format")?;
                Ok(SystemTime::from(dt))      
            },
            None => Err(anyhow!("EXIF data does not contain a created date"))
        }
    }

    fn amend_file_date(dt: SystemTime, file_path: &Path) -> anyhow::Result<()> {

        // Prepare the date to be used on the file
        let filetimes = FileTimes::new().set_modified(dt);

        // Change the 'file modified' date to the found metadata date
        let file_to_amend = File::options().write(true).open(file_path)?;
        file_to_amend.set_times(filetimes)?;
        println!("Converted: {}", file_path.display());
        Ok(())
    }