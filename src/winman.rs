#[macro_use]
extern crate lazy_static;

extern crate winapi;
extern crate comctl32;
extern crate kernel32;
extern crate user32;
extern crate gdi32;
extern crate spmc;
extern crate fuzzy;

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
    let windows::popup::ManagedWindow(_, ref popup) = PopupWindow::new()
        .expect("Popup creation failed");

    let rx = popup.borrow().listen();

    {
        let popup = popup.borrow();
        popup.show();
    }

    // Persistent state    
    let mut window_list = Vec::new();

    let mut msg: MSG = MSG {
        hwnd: hwnd,
        message: 0,
        wParam: 0 as WPARAM,
        lParam: 0 as LPARAM,
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };

    while unsafe { user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) } > 0 {
        unsafe {
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }

        while let Ok(event) = rx.try_recv() {
            use windows::messages::PopupMsg;

            match event {
                PopupMsg::Show => {
                    window_list.clear();
                    get_window_list(&mut window_list);
                    println!("Grabbed {} window titles", window_list.len());
                },

                PopupMsg::Search(Some(s)) => {
                    println!("Search: {}", s);

                    // <_habnabit>	salad, extern fn exportable<F>(mut func: F) where F: FnMut ... { func() }, then pass_to_ffi(|| do_whatever())
                    // salad: Even for wndproc, you can store user data in the window itself
                    // SetWindowLongPtr though, because 64bit is a thing
                    // https://github.com/retep998/wio-rs/blob/master/src/apc.rs
                    // https://github.com/retep998/wio-rs/blob/master/src/wide.rs
                    // https://msdn.microsoft.com/en-us/library/windows/desktop/ms633497(v=vs.85).aspx
                    // use mem::Zero() instead of default::Default()
                    // WindowsBunny on IRC
                },

                PopupMsg::Search(None) => {
                    println!("Search: <null>");
                },

                PopupMsg::Accept(s) => {
                    println!("Accept: {}", s);

                    let finder = fuzzy::Finder::new(&s).unwrap();

                    let xx = window_list.iter().find(|w| finder.is_match(&w.1));
                    match xx {
                        Some(&(hwnd, ref title)) => {
                            println!("match! {:?} {}", hwnd, title);
                            let _ = window_tracking::set_foreground_window(hwnd);
                            // popup.borrow()._hide();
                        },
                        None => println!("no match!")
                    }
                }
            }
        }
    }
}

fn get_window_list(vec: &mut Vec<(HWND, String)>) {
    const BUFFER_LEN: usize = 1024;
    let mut buffer = [0u16; BUFFER_LEN];

    enum_windows(|hwnd| {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        // TODO: dynamic buffer with GetWindowTextLength
        // The return value, however, will always be at least as large as the actual
        // length of the text; you can thus always use it to guide buffer allocation
        // (https://msdn.microsoft.com/en-us/library/windows/desktop/ms633521(v=vs.85).aspx)
        let len = unsafe { user32::GetWindowTextW(hwnd, buffer.as_mut_ptr(), BUFFER_LEN as i32) };

        // https://gist.github.com/sunnyone/e660fe7f73e2becd4b2c
        if len > 0 {
            let null = buffer.iter().position(|x| *x == 0).unwrap_or(BUFFER_LEN);
            let slice = unsafe { std::slice::from_raw_parts(buffer.as_ptr(), null) };
            let text = OsString::from_wide(slice).to_string_lossy().into_owned();

            vec.push((hwnd, text));
        }

        TRUE
    }).expect("Callback does not SetLastError");
}

// https://github.com/retep998/wio-rs/blob/master/src/apc.rs
fn enum_windows<T>(func: T) -> Win32Result<()>
    where T: FnMut(HWND) -> BOOL {

    unsafe extern "system" fn helper<T: FnMut(HWND) -> BOOL>(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ppfn = lparam as *mut T;
        let mut func = &mut *ppfn;

        func(hwnd)
    }

    let result = unsafe {
        let ppfn = (&func) as *const T;
        user32::EnumWindows(Some(helper::<T>), ppfn as LPARAM)
    };

    match result {
        FALSE => match unsafe { kernel32::GetLastError() } {
            0 => Ok(()),
            err => Err(err)
        },
        _ => Ok(())
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
