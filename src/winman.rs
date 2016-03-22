extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use winapi::winuser::*;

type Win32Result<T> = Result<T, DWORD>;

pub fn main() {
	println!("Hello Windows!");

    // API demo
    unsafe {
        let foreground_window = user32::GetForegroundWindow();
        println!("{:?}", foreground_window);

        user32::ShowWindow(foreground_window, SW_HIDE);
        kernel32::Sleep(1000);
        user32::ShowWindow(foreground_window, SW_SHOW);
    }

    // Window creation
    unsafe {
        let hwnd = create_window(Some(window_proc)).expect("Window creation failed");
        let mut msg: MSG = MSG {
            hwnd: hwnd,
            message: 0,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        while user32::GetMessageW(&mut msg, hwnd, 0, 0) > 0 {
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);

            // Hotkeys are sent to the thread, not the window, so they
            // cannot be handled in WNDPROC
            // if msg.message == WM_HOTKEY {
            //     let modifiers = LOWORD(msg.lParam as DWORD) as UINT;
            //     let vk = HIWORD(msg.lParam as DWORD) as UINT;

            //     hotkey_manager.process_hotkey((modifiers, vk));
            // }
        }
    }

}

unsafe fn create_window(window_proc: WNDPROC) -> Win32Result<HWND> {
	let class_name: Vec<u16> = OsStr::new("MyMagicClassName").encode_wide().collect();

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

    Ok(hwnd)
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let lresult = match msg {
        WM_CREATE => Some(0),
        
        WM_DESTROY => {
            user32::PostQuitMessage(0);
            Some(0)
        },
        
        WM_COMMAND => {
            let command = LOWORD(wparam as DWORD);

            // Tray commands
            if command == 1 {
                user32::DestroyWindow(hwnd);
            }

            Some(0)
        },
        
        user if user >= WM_USER => Some(0),
        
        _ => None
    };

    lresult.unwrap_or(user32::DefWindowProcW(hwnd, msg, wparam, lparam))
}