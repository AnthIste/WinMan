#![feature(globs)]

extern crate libc;

use std::default::Default;

use win32::constants::*;
use win32::types::{HWND,MSG,UINT,DWORD,WNDPROC,WPARAM,LPARAM,LRESULT};
use win32::window::{MessageBoxA,GetMessageW,TranslateMessage,DispatchMessageW,DefWindowProcW};
use win32::macro::{LOWORD,HIWORD};

use app::window::{Win32Window};
use app::dummy::{DummyWindow};
use app::hotkey::{HotkeyManager};

// Consider moving to crate win32
mod win32;
mod app;

static mut s_dummy_window: Option<DummyWindow> = None;

fn main() {
    // https://github.com/rust-lang/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    // Potential macro to handle Option<T> failures:
    // `macro_rules! try_option {($x:expr) => (match $x {Some(x) => x, None => return})}`
    // Otherwise use try! with Result<T, E>

    let create_result = DummyWindow::create(None, main_wnd_proc as WNDPROC);

    match create_result {
        Ok(dummy_window) => {
            // How better to map static WinMain? A channel?
            unsafe { s_dummy_window = Some(dummy_window); }

            let hotkey_manager = HotkeyManager::new();            
            
            hotkey_manager.register_hotkeys();

            let mut msg: MSG = Default::default();

            while GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);

                // Hotkeys are sent to the thread, not the window
                if msg.message == WM_HOTKEY {
                    let modifiers = LOWORD(msg.lParam as DWORD) as UINT;
                    let vk = HIWORD(msg.lParam as DWORD) as UINT;

                    hotkey_manager.process_hotkey((modifiers, vk));
                }
            }

            MessageBoxA(0 as HWND, "All done!".to_c_str().as_ptr(), "Exiting".to_c_str().as_ptr(), 0);
        }

        Err(code) => {
            MessageBoxA(0 as HWND, format!("We couldn't create a window becase of {:X} :<", code).to_c_str().as_ptr(), "Exiting".to_c_str().as_ptr(), 0);
        }
    }
}

extern "system" fn main_wnd_proc(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        let mut window = s_dummy_window.unwrap();

        let handled = match msg {
            win32::constants::WM_CREATE => {
                window.on_create()
            }

            win32::constants::WM_DESTROY => {
                window.on_destroy()
            }

            win32::constants::WM_COMMAND => {
                window.on_command(LOWORD(wParam as DWORD))
            }

            user_command if user_command >= WM_USER => {
                window.on_user(msg, wParam, lParam)
            }

            _ => {
                None
            }
        };

        match handled {
            Some(b) => {
                if b { 0 } else { 1 }
            }

            None => {
                DefWindowProcW(hWnd, msg, wParam, lParam)
            }
        }
    }
}