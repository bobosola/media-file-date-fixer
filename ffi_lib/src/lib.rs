use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use mfdf::{fix_dates, Report};

/// FFI function called from the other side in Swift, C#, etc.
/// - Expects a UTF8 directory path as input
/// - Returns a report string from the fix_dates() function
/// - Caller must free the returned pointer using free_string()
#[unsafe(no_mangle)]
pub extern "C" fn make_report(raw_path: *const c_char) -> *mut c_char {
    if raw_path.is_null() {
        return std::ptr::null_mut();
    }

    let cstr = unsafe { CStr::from_ptr(raw_path) };

    let utf8 = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let path = Path::new(utf8);
    let report: Report = fix_dates(path);

    let report_string = format!(
        "mfdf report for files in {:?}\n\
         examined: {}\n\
         ignored: {}\n\
         updated: {}\n\
         failed: {}\n",
        path, report.examined, report.ignored, report.updated, report.failed
    );

    let mut full_report = report_string;

    if !report.err_msgs.is_empty() {
        full_report.push_str("\nfailure details:\n");
        full_report.push_str(&report.err_msgs.join("\n"));
    }

    CString::new(full_report)
        .map(|cs| cs.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// FFI function called from the other side in Swift, C#, etc.
/// Required to free the memory allocated for the report
#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    }
}
