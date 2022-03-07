/// Example with integer
#[no_mangle]
fn answer() -> u32 {
    42
}

/// Example with string
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
fn hi_mate() -> *const c_char {
    let c_to_print = CString::new("Ho hi, mate!").expect("CString::new failed");
    c_to_print.into_raw()
}

#[no_mangle]
fn hi_mate_len() -> usize {
    "Ho hi, mate!".len()
}