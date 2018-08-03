mod glfw;
pub use self::glfw::*;

use std::borrow::Cow;
use std::ffi::{ CStr, CString };

use libc::c_char;

pub fn to_cstring(string: &str) -> CString {
    CString::new(string).unwrap()
}

pub fn from_cstring<'a>(string: *const c_char) -> Cow<'a, str> {
    unsafe { CStr::from_ptr(string) }.to_string_lossy()
}
