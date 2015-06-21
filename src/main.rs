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

unsafe fn get_instance() -> winapi::HINSTANCE {
    let instance = kernel32::GetModuleHandleW(ptr::null());
    if instance.is_null() {
        panic!("GetModuleHandleW error: {}", kernel32::GetLastError());
    }

    instance
}

unsafe fn register_class(instance: winapi::HINSTANCE) -> winapi::ATOM {
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
    let atom = user32::RegisterClassW(&class);
    if atom == 0 {
        panic!("RegisterClassW error: {}", kernel32::GetLastError());
    }

    atom
}

unsafe fn create_window(instance: winapi::HINSTANCE, class: winapi::ATOM) -> winapi::HWND {
    let window = user32::CreateWindowExW(
        0,
        class as winapi::LPCWSTR,
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
    if window.is_null() {
        panic!("CreateWindowExW error: {}", kernel32::GetLastError());
    }

    window
}

unsafe fn get_message(message: &mut winapi::MSG) -> winapi::BOOL {
    let status = user32::GetMessageW(message, ptr::null_mut(), 0, 0);
    if status == -1 {
        panic!("GetMessageW error: {}", kernel32::GetLastError());
    }

    status
}

unsafe fn messageLoop() {
    let instance = get_instance();
    let class = register_class(instance);
    let window = create_window(instance, class);

    let mut message = winapi::MSG {
        hwnd: ptr::null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: winapi::POINT {
            x: 0,
            y: 0
        }
    };
    loop {
        let status = get_message(&mut message);
        if status == 0 {
            break;
        }

        user32::TranslateMessage(&message);
        user32::DispatchMessageW(&message);
    }
}

unsafe extern "system" fn wndProc(
    window: winapi::HWND,
    message: winapi::UINT,
    wParam: winapi::WPARAM,
    lParam: winapi::LPARAM) -> winapi::LRESULT {
    println!("Message received: {}", message);
    0
}
