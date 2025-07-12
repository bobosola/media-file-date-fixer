use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use mfdf::{fix_dates, Report};

// Called from the other side in Swift, C#, whatever
#[unsafe(no_mangle)]
pub extern "C" fn make_report(input: *const c_char) -> *mut c_char {

    // Convert the user-supplied C path string to a Rust String
    // NB: a non-UTF-8 path will fail and be picked up in the report
    let c_path = unsafe {
        let c_path = CStr::from_ptr(input);
        c_path.to_string_lossy().to_owned()
    };

    // Fix the dates in the supplied path and generate a report
    let report: Report = fix_dates(c_path.as_ref());

    // Return the report text as a C string
    let mut report_string = format!(
        "mfdf report for files in {}:\n \
        • examined: {}\n \
        • updated: {}\n \
        • failed: {}\n", c_path, report.examined, report.updated, report.errors);

    if !report.err_msgs.is_empty() {
        let newline = String::from("\n");
        report_string.push_str(&newline);
        report_string.push_str(&String::from("Failure details:"));
        for error_msg in &report.err_msgs {
            report_string.push_str(&newline);
            report_string.push_str(error_msg);
        }
    }

    // Convert to a C string and return (the caller is responsible for freeing)
    let c_report = CString::new(report_string).expect("CString::new failed");
    c_report.into_raw()
}

// Called from the other side after getting the report to prevent memory leaks
#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    }
}
