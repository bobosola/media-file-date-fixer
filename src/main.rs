use std::process:: exit;
use std::env;
use media_file_date_corrector::{fix_dates, Report};

// An application to fix the lost Created and Modified file dates in copied media files
fn main() -> () {

    // Get a directory path as the single argument
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Media File Date Fixer (mfdf)");
        println!("Please provide a parent directory path containing media files");
        println!("Usage: ./mfdf \"/Users/bob/media files\"");
        exit(0);
    }
    let dir_path = &args[1];

    // The output report on completion
    let report = &mut Report {
        files_examined: 0,
        files_updated: 0,
        files_with_errors: 0,
        errors: vec![]
    };

    let results = fix_dates(dir_path, report);

    println!("\nReport for {}", dir_path);
    println!("Files examined:    {}", results.files_examined);
    println!("Files updated:     {}", results.files_updated);
    println!("Files with errors: {}\n", results.files_with_errors);
    if !results.errors.is_empty() {
        println!("Error details:");
        for str_error_msg in &results.errors {
            println!("{}", str_error_msg);
        }
    }
}
