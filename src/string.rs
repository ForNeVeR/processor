use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn to_wchar(str : &str) -> Vec<u16> {
    OsStr::new(str).encode_wide(). chain(Some(0).into_iter()).collect()
}
