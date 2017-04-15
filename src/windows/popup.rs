use std;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use kernel32;
use user32;
use winapi::*;

use utils::Win32Result;

pub struct PopupWindow {
    hwnd: HWND
}

impl PopupWindow {
    fn new(hwnd: HWND) -> Self {
        PopupWindow {
            hwnd: hwnd
        }
    }

    pub fn show(&mut self) {
        // TODO: SendMessage
    }

    pub fn hide(&mut self) {
        // TODO: SendMessage
    }
}

pub fn create_window() -> Win32Result<PopupWindow> {
    let create_result = create_window_impl(Some(window_proc));

    match create_result {
        Ok(hwnd) => Ok(PopupWindow::new(hwnd)),
        Err(e) => Err(e),
    }
}

fn create_window_impl(window_proc: WNDPROC) -> Win32Result<HWND> {
    let class_name: Vec<u16> = OsStr::new("WinmanMainWindow").encode_wide().collect();
    
    let hwnd = unsafe {
        let window_class = WNDCLASSEXW {
        	cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        	style: 0,
        	lpfnWndProc: window_proc,
        	cbClsExtra: 0,
        	cbWndExtra: 0,
        	hInstance: 0 as HINSTANCE,
        	hIcon: 0 as HICON,
        	hCursor: 0 as HCURSOR,
        	hbrBackground: 0 as HBRUSH,
        	lpszMenuName: 0 as LPCWSTR,
        	lpszClassName: class_name.as_ptr(),
        	hIconSm: 0 as HICON,
        };

        if user32::RegisterClassExW(&window_class) == 0 {
            return Err(kernel32::GetLastError());
        }

        let hwnd = user32::CreateWindowExW(
            0,
            class_name.as_ptr(),
            0 as LPCWSTR,
            0,
            0,
            0,
            0,
            0,
            0 as HWND,
            0 as HMENU,
            0 as HINSTANCE,
            0 as LPVOID);

        if hwnd == 0 as HWND {
            return Err(kernel32::GetLastError());
        }

        hwnd
    };

    Ok(hwnd)
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let lresult = match msg {
        WM_HOTKEY => {
            let _modifiers = LOWORD(lparam as DWORD);
            let _vk = HIWORD(lparam as DWORD);

            Some(0)
        },

        WM_COMMAND => {
            let _command = LOWORD(wparam as DWORD);

            Some(0)
        },

        WM_DESTROY => {
            user32::PostQuitMessage(0);
            Some(0)
        },
        
        user if user >= WM_USER => Some(0),
        
        _ => None
    };

    lresult.unwrap_or(user32::DefWindowProcW(hwnd, msg, wparam, lparam))
}