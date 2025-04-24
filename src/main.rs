use std::process:: exit;
use std::env;
use media_file_date_corrector::{fix_dates, Report};

// A CLI runner for the media_file_date_corrector library
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

    // Prepare an empty output report
    let report = &mut Report {
        examined: 0,
        updated: 0,
        failed: 0,
        errors: vec![]
    };

    let results = fix_dates(dir_path, report);

    println!("\nmfdf report for files in {}:\n", dir_path);
    println!("examined:    {}", results.examined);
    println!("updated:     {}", results.updated);
    println!("errors:      {}\n", results.failed);
    if !results.errors.is_empty() {
        println!("error details:");
        for str_error_msg in &results.errors {
            println!("{}", str_error_msg);
        }
    }
}
