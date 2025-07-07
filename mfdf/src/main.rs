use std::{ env, process::exit };
use mfdf::fix_dates;

// Simple runner for the media_file_date_fixer library
fn main() -> () {

    let args: Vec<String> = env::args().collect();
    let num_args = args.len();
    if num_args == 1 || (num_args > 1 && ["help", "--help", "-h", "-?", "/?"].iter().any(|h| h == &args[1])) {
        print!("\n \
            ---------------- Media File Date Fixer (mfdf) ----------------\n \
            This app can recover:\n \
            • lost 'Created' & 'Modified' dates for copied image files\n \
            • lost 'Created' dates for copied video files\n \
            It works with most common photo and video formats.\n\n \
            It requires a directory path as its single argument and will\n \
            traverse all sub-directories. Example usage: \n \
            • ./mfdf ~/Desktop/copiedfiles\n \
            • ./mfdf \'/Users/bob/Desktop/copied files'\n \
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
    ", dir_path, report.examined, report.updated, report.errors);

    if !report.err_msgs.is_empty() {
        println!("\nerror details:");
        for error_msg in &report.err_msgs {
            println!("{}", error_msg);
        }
    }
}
