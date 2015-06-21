extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ptr;

mod string;
mod window;

fn main() {
    unsafe { message_loop(); }
}

unsafe fn get_message(message: &mut winapi::MSG) -> winapi::BOOL {
    let status = user32::GetMessageW(message, ptr::null_mut(), 0, 0);
    if status == -1 {
        panic!("GetMessageW error: {}", kernel32::GetLastError());
    }

    status
}

unsafe fn message_loop() {
    window::create_window(Some(wnd_proc));

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

unsafe extern "system" fn wnd_proc(
    window: winapi::HWND,
    message: winapi::UINT,
    wParam: winapi::WPARAM,
    lParam: winapi::LPARAM) -> winapi::LRESULT {
    println!("Message received: {}", message);
    0
}
