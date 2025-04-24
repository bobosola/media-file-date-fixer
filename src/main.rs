use std::process:: exit;
use std::env;
use media_file_date_corrector:: fix_dates;

// Simple runner for the media_file_date_corrector library
fn main() -> () {

    // Get a directory path as the single argument
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("\nMedia File Date Fixer (mfdf)");
        println!("Please provide a parent directory path containing media files");
        println!("Example usage: ./mfdf /Users/<username>/mediafiles\n");
        exit(0);
    }
    let dir_path = &args[1];
    let report = fix_dates(dir_path);

    println!("\nmfdf report for files in {}:\n", dir_path);
    println!("examined: {}", report.examined);
    println!("updated:  {}", report.updated);
    println!("errors:   {}\n", report.failed);
    if !report.errors.is_empty() {
        println!("error details:");
        for error_msg in &report.errors {
            println!("{}", error_msg);
        }
    }
}
