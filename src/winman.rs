extern crate winapi;
extern crate kernel32;
extern crate user32;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winuser::*;

pub fn main() {
	println!("Hello Windows!");

	let foreground_window = unsafe { user32::GetForegroundWindow() };

	println!("{:?}", foreground_window);

	unsafe {
		user32::ShowWindow(foreground_window, SW_HIDE);
		kernel32::Sleep(1000);
		user32::ShowWindow(foreground_window, SW_SHOW);
	}
}

extern "system" fn _window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        let lresult = match msg {
            WM_CREATE               => Some(0),
            WM_DESTROY              => Some(0),
            WM_COMMAND              => Some(0),
            user if user >= WM_USER => Some(0),
            _                       => None
        };

        lresult.unwrap_or(user32::DefWindowProcW(hwnd, msg, wparam, lparam))
    }
}
