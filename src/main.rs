extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn main() {
    unsafe { messageLoop(); }
}

fn to_wchar(str : &str) -> Vec<u16> {
    OsStr::new(str).encode_wide(). chain(Some(0).into_iter()).collect()
}

unsafe fn messageLoop() -> i32 {
    let instance = kernel32::GetModuleHandleW(ptr::null());
    let class = winapi::WNDCLASSW {
        style: 0,
        lpfnWndProc: Some(wndProc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: ptr::null_mut(),
        hCursor: ptr::null_mut(),
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null_mut(),
        lpszClassName: to_wchar("HiddenWindowClass").as_ptr()
    };
    let classAtom = user32::RegisterClassW(&class);
    let window = user32::CreateWindowExW(
        0,
        classAtom as winapi::LPCWSTR,
        ptr::null_mut(),
        0,
        0,
        0,
        0,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
        instance,
        ptr::null_mut());

    0
}

unsafe extern "system" fn wndProc(
    window: winapi::HWND,
    message: winapi::UINT,
    wParam: winapi::WPARAM,
    lParam: winapi::LPARAM) -> winapi::LRESULT {
    println!("Message received: {}", message);
    0
}
