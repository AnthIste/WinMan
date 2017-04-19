#[macro_use]
extern crate lazy_static;

extern crate winapi;
extern crate comctl32;
extern crate kernel32;
extern crate user32;
extern crate gdi32;
extern crate spmc;

mod constants;
mod utils;
mod window_tracking;
mod windows;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::Mutex;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use winapi::winuser::*;

use constants::*;
use utils::Win32Result;
use window_tracking::Config;
use windows::popup::PopupWindow;

// Hotkey modifiers
const MOD_APPCOMMAND: UINT = MOD_CONTROL | MOD_ALT;
const MOD_GRAB_WINDOW: UINT = MOD_ALT | MOD_SHIFT;
const MOD_SWITCH_WINDOW: UINT = MOD_ALT;
const MOD_CLEAR_WINDOWS: UINT = MOD_CONTROL | MOD_ALT;

// Runtime data - everything is static
lazy_static! {
    static ref CONFIG: Mutex<Config> = {
        let config = load_config().unwrap_or(Config::new());
        
        Mutex::new(config)
    };
}

fn load_config() -> Option<Config> {
    Some(Config::new())
}

pub fn main() {
	println!("Hello Windows!");

    // Main window
    let hwnd = create_window(Some(window_proc)).expect("Window creation failed");
    register_hotkeys(hwnd);

    // Popup window
    let popup = PopupWindow::new()
        .expect("Popup creation failed");

    let rx = popup.borrow().listen();

    {
        let popup = popup.borrow();
        popup.show();
    }
    
    let mut msg: MSG = MSG {
        hwnd: hwnd,
        message: 0,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };

    unsafe {
        while user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);

            use windows::messages::PopupMsg;
            while let Ok(event) = rx.try_recv() {
                match event {
                    PopupMsg::Search(s) => {
                        println!("Search: {}", s);
                    }
                }
            }
        }
    }
}

fn create_window(window_proc: WNDPROC) -> Win32Result<HWND> {
    let class_name: Vec<u16> = OsStr::new("WinmanMainWindow")
        .encode_wide()
        .chain(::std::iter::once(0))
        .collect();
    
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

fn register_hotkeys(hwnd: HWND) {
    // Virtual key codes: https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
    // CTRL-ALT-Q to quit
    unsafe { user32::RegisterHotKey(hwnd, 0, MOD_APPCOMMAND, VK_Q); }

    // ALT-SHIFT-1 to ALT-SHIFT-9 to grab windows,
    // ALT-1 to ALT-9 to switch windows
    for i in 0..10 {
        let vk_n = VK_0 + i;

        unsafe {
            user32::RegisterHotKey(hwnd, 1, MOD_GRAB_WINDOW, vk_n);
            user32::RegisterHotKey(hwnd, 2, MOD_SWITCH_WINDOW, vk_n);
            user32::RegisterHotKey(hwnd, 3, MOD_CLEAR_WINDOWS, vk_n);
        }
    }
}

fn on_hotkey(modifiers: UINT, vk: UINT) -> Option<LRESULT> {
    match (modifiers, vk) {
        // Hotkey: Quit
        (MOD_APPCOMMAND, VK_Q) => {
            unsafe { user32::PostQuitMessage(0); }
            Some(0)
        },

        // Hotkey: Grab a window
        (MOD_GRAB_WINDOW, vk) => {
            let mut config = CONFIG.lock().unwrap();
            let window = window_tracking::get_foreground_window();

            if let Ok(window) = window {
                println!("Tracking foreground window {:?}: {}",
                    window.hwnd(),
                    window.title().unwrap_or("No title"));
                
                config.track_window(vk, window);
            }

            Some(0)
        },

        // Hotkey: Switch to a grabbed window
        (MOD_SWITCH_WINDOW, _vk) => {
            let mut config = CONFIG.lock().unwrap();
            let window_set = config.get_windows(vk);

            if let Some(window_set) = window_set {
                while let Some(window) = window_set.cycle() {
                    println!("Switching to window {:?}: {}",
                        window.hwnd(),
                        window.title().unwrap_or("No title"));

                    match window_tracking::set_foreground_window(window.hwnd()) {
                        Ok(_) => break,
                        Err(_) => {
                            window_set.remove(&window);
                        }
                    }
                }
            }

            Some(0)
        },

        // Hotkey: Clear windows
        (MOD_CLEAR_WINDOWS, vk) => {
            let mut config = CONFIG.lock().unwrap();

            println!("Clearing windows on hotkey {}", vk);
            config.clear_windows(vk);

            Some(0)
        },

        _ => None
    }
}

fn on_command(hwnd: HWND, command: UINT) -> Option<LRESULT> {
    match command {
        1 => {
            unsafe { user32::DestroyWindow(hwnd); }
            Some(0)
        },
        
        _ => None
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let lresult = match msg {
        WM_HOTKEY => {
            let modifiers = LOWORD(lparam as DWORD);
            let vk = HIWORD(lparam as DWORD);

            on_hotkey(modifiers as UINT, vk as UINT)
        },

        WM_COMMAND => {
            let command = LOWORD(wparam as DWORD);
            
            on_command(hwnd, command as UINT)
        },

        WM_DESTROY => {
            user32::PostQuitMessage(0);
            Some(0)
        },
        
        user if user >= WM_USER => Some(0),
        
        _ => None
    };

    lresult.unwrap_or_else(|| user32::DefWindowProcW(hwnd, msg, wparam, lparam))
}
