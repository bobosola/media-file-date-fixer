use std::{ env, process::exit, path::Path };
use mfdf::fix_dates;

// Simple runner for the media_file_date_fixer library
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            print_help();
            exit(0);
        }
        2 => {
            if ["help", "--help", "-h", "-?", "/?"].contains(&args[1].as_str()) {
                print_help();
                exit(0);
            }
        }
        _ => {
            eprintln!("Error: Too many arguments provided");
            print_help();
            exit(1);
        }
    }

    let dir_path = Path::new(&args[1]);

    if !dir_path.exists() {
        eprintln!("Error: Directory '{}' does not exist", dir_path.display());
        exit(1);
    }

    if !dir_path.is_dir() {
        eprintln!("Error: '{}' is not a directory", dir_path.display());
        exit(1);
    }

    println!("Processing files in: {}", dir_path.display());
    let report = fix_dates(dir_path);

    print_report(&report, dir_path);
}

fn print_help() {
    println!(
        r#"
---------------- Media File Date Fixer (mfdf) ----------------
This app can recover:
• lost 'Created' & 'Modified' dates for copied image files
• lost 'Created' dates for copied video files
It works with most common photo and video formats.

It requires a directory path as its single argument and will
traverse all sub-directories. Example usage:
• ./mfdf ~/Desktop/copiedfiles
• ./mfdf '/Users/bob/Desktop/copied files'
        "#
    );
}

fn print_report(report: &mfdf::Report, dir_path: &Path) {
    println!();
    println!("mfdf report for files in: {}", dir_path.display());
    println!("  examined: {}", report.examined);
    println!("  ignored:  {}", report.ignored);
    println!("  updated:  {}", report.updated);
    println!("  errors:   {}", report.failed);

    if !report.err_msgs.is_empty() {
        println!("\nerror details:");
        for (i, error_msg) in report.err_msgs.iter().enumerate() {
            println!("  {}. {}", i + 1, error_msg);
        }
    }
}
