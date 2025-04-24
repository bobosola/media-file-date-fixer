
use std::{ env, process::exit };
use media_file_date_corrector::fix_dates;

/// Simple runner for the media_file_date_corrector library
fn main() -> () {

    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len == 1 || (args_len > 1 && ["help", "--help", "-h"].iter().any(|&h| h == args[1])) {
        print!("\n \
            -------------- Media File Date Fixer (mfdf) --------------\n \
            Recreates the original 'Created' date for copied video and image files.\n \
            Modified dates are also recreated for copied image files.\n\n \
            Requires a directory path as the single argument.\n \
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
