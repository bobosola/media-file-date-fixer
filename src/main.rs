
use std::{ env, process::exit };
use media_file_date_corrector::fix_dates;

/// Simple runner for the media_file_date_corrector library
fn main() -> () {

    let args: Vec<String> = env::args().collect();
    let num_args = args.len();
    if num_args == 1 || (num_args > 1 && ["help", "--help", "-h", "-?", "/?"].iter().any(|&h| h == args[1])) {
        print!("\n \
            -------------- Media File Date Fixer (mfdf) --------------\n \
            This app retrieves:\n \
             • lost 'Created' & 'Modified' dates for copied image files\n \
             • lost 'Created' dates for copied video files\n \
            It requires a directory path as its single argument.\n \
            It fixes all supported media files in that directory and all subdirectories.\n\n \
            Example usage: ./mfdf ~/Desktop/copiedfiles\n \
        \n");
        exit(0);
    }

    let dir_path = &args[1];
    let report = fix_dates(dir_path);

    print!("\n \
        mfdf report for files in {}:\n \
        examined: {}\n \
        updated:  {}\n \
        errors:   {}\n \
    ",  dir_path, report.examined, report.updated, report.failed);

    if !report.errors.is_empty() {
        println!("\nerror details:");
        for error_msg in &report.errors {
            println!("{}", error_msg);
        }
    }
}
