extern crate winapi;
extern crate comctl32;
extern crate kernel32;
extern crate user32;
extern crate gdi32;
extern crate spmc;
extern crate fuzzy;

use winapi::minwindef::*;
use winapi::windef::*;

use utils::Win32Result;
use window_tracking::Config;
use windows::main::{AppWindow, AppMsg};
use windows::popup::{PopupWindow, PopupMsg};

mod constants;
mod utils;
mod window_tracking;
mod windows;

pub fn main() {
	println!("Hello Windows!");

    // Register window classes
    AppWindow::register_classes().expect("Could not register AppWindow class");
    PopupWindow::register_classes().expect("Could not register PopupWindow class");

    // Main window
    let app_window = AppWindow::new().expect("Could not create AppWindow");
    let app_rx = app_window.listen();

    // Popup window
    let popup = PopupWindow::new().expect("Could not create PopupWindow");
    let popup_rx = popup.listen();

    // Persistent state
    let mut config = load_config().unwrap_or(Config::new());
    let mut window_list: Vec<(HWND, String)> = Vec::new();

    let mut msg = unsafe { ::std::mem::zeroed() };
    while unsafe { user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) } > 0 {
        unsafe {
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }

        // App messages
        while let Ok(event) = app_rx.try_recv() {
            match event {
                AppMsg::ShowPopup => {
                    window_list.clear();
                    get_window_list(&mut window_list);
                    println!("Grabbed {} window titles", window_list.len());

                    popup.show();
                },

                AppMsg::GrabWindow(vk) => {
                    let window = window_tracking::get_foreground_window();

                    if let Ok(window) = window {
                        println!("Tracking foreground window {:?}: {}",
                            window.hwnd(),
                            window.title().unwrap_or("No title"));
                        
                        config.track_window(vk, window);
                    }
                },

                AppMsg::FocusWindow(vk) => {
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
                },

                AppMsg::ClearWindow(vk) => {
                    println!("Clearing windows on hotkey {}", vk);
                    config.clear_windows(vk);
                },
            }
        }

        // Popup messages
        while let Ok(event) = popup_rx.try_recv() {
            match event {
                PopupMsg::Search(Some(s)) => {
                    println!("Search: {}", s);
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
                            popup._hide();
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

fn load_config() -> Option<Config> {
    Some(Config::new())
}