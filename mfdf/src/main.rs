use std::path::PathBuf;
use clap::Parser;
use mfdf::fix_dates;

/// Media File Date Fixer - recover 'Created' dates for copied media files
#[derive(Parser)]
#[command(name = "mfdf")]
#[command(about = "Recover 'Created' dates for copied image and video files")]
#[command(version)]
struct Args {
    /// Directory path to process (will traverse subdirectories)
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let dir_path = &args.path;

    if !dir_path.exists() {
        eprintln!("Error: '{}' does not exist", dir_path.display());
        std::process::exit(1);
    }

    if !dir_path.is_dir() {
        eprintln!("Error: '{}' is not a directory", dir_path.display());
        std::process::exit(1);
    }

    println!("Processing files in: {}", dir_path.display());
    let report = fix_dates(dir_path);
    print_report(&report, dir_path);
}

fn print_report(report: &mfdf::Report, dir_path: &PathBuf) {
    println!();
    println!("mfdf report for files in: {}", dir_path.display());
    println!("  examined:   {}", report.examined);
    println!("  updated:    {}", report.updated);
    println!("  failed:     {}", report.failed);
    

    if !report.err_msgs.is_empty() {
        println!("\nfailure details:");
        for (i, error_msg) in report.err_msgs.iter().enumerate() {
            println!("  {}. {}", i + 1, error_msg);
        }
    }
    println!("{}", report.time_taken);
}
