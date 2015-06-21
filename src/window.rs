extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ptr;

use string;

const DUMMY_CLASS_NAME: &'static str = "processor_class";
const HWND_MESSAGE_: isize = -3;
const HWND_MESSAGE: winapi::HWND = HWND_MESSAGE_ as winapi::HWND;

unsafe fn get_instance() -> winapi::HINSTANCE {
    let instance = kernel32::GetModuleHandleW(ptr::null());
    if instance.is_null() {
        panic!("GetModuleHandleW error: {}", kernel32::GetLastError());
    }

    instance
}

unsafe fn register_class(instance: winapi::HINSTANCE, wnd_proc: winapi::WNDPROC) {
    let class = winapi::WNDCLASSW {
        style: 0,
        lpfnWndProc: wnd_proc,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: ptr::null_mut(),
        hCursor: ptr::null_mut(),
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null_mut(),
        lpszClassName: string::to_wchar(DUMMY_CLASS_NAME).as_ptr()
    };
    let atom = user32::RegisterClassW(&class);
    if atom == 0 {
        panic!("RegisterClassW error: {}", kernel32::GetLastError());
    }
}

pub unsafe fn create_window(wnd_proc: winapi::WNDPROC) -> winapi::HWND {
    let instance = get_instance();
    register_class(instance, wnd_proc);

    let window = user32::CreateWindowExW(
        0,
        string::to_wchar(DUMMY_CLASS_NAME).as_ptr(),
        ptr::null_mut(),
        0,
        0,
        0,
        0,
        0,
        HWND_MESSAGE,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut());
    if window.is_null() {
        panic!("CreateWindowExW error: {}", kernel32::GetLastError());
    }

    window
}
