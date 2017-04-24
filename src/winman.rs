extern crate winapi;
extern crate comctl32;
extern crate kernel32;
extern crate user32;
extern crate gdi32;
extern crate spmc;
extern crate fuzzy;

use winapi::minwindef::*;
use winapi::windef::*;

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
    let popup = PopupWindow::new(app_window.hwnd).expect("Could not create PopupWindow");
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
                PopupMsg::Search(_) => {},

                PopupMsg::Accept(s) => {
                    use fuzzy::FuzzyResult;

                    println!("Accept: {}", s);

                    // Iterate open windows and determine match strength
                    let finder = fuzzy::Finder::new(&s).unwrap();
                    let mut matches: Vec<_> = window_list.iter()
                        .map(|w| {
                            let (hwnd, ref title) = *w;
                            let m = finder.is_match(&title);

                            (hwnd, title, m)
                        })
                        .filter(|m| m.2 != FuzzyResult::None)
                        .collect();

                    // Sort by FuzzyResult (strength)
                    matches.sort_by_key(|m| m.2);

                    match matches.first() {
                        Some(&(hwnd, ref title, m)) => {
                            println!("match! {:?} {}", m, title);
                            let _ = window_tracking::set_foreground_window(hwnd);
                            popup._hide();
                        },
                        _ => println!("no match!")
                    }
                }
            }
        }
    }
}

fn get_window_list(vec: &mut Vec<(HWND, String)>) {
    utils::api_wrappers::enum_windows(|hwnd| {
        let is_visible = unsafe { user32::IsWindowVisible(hwnd) };

        if is_visible != 0 {
            if let Ok(text) = utils::api_wrappers::get_window_text(hwnd) {
                vec.push((hwnd, text));
            }
        }

        TRUE
    }).expect("Callback does not SetLastError");
}

fn load_config() -> Option<Config> {
    Some(Config::new())
}