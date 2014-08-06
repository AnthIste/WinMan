#![feature(globs)]

extern crate libc;

use std::default::Default;

use win32::constants::*;
use win32::types::{HWND,MSG,UINT,DWORD,WNDPROC,WPARAM,LPARAM,LRESULT};
use win32::window::{MessageBoxA,GetMessageW,TranslateMessage,DispatchMessageW,PostQuitMessage,
                    DefWindowProcW,DestroyWindow};
use win32::macro::{LOWORD,HIWORD};

use app::dummy::{DummyWindow};
use app::hotkey::{HotkeyManager};

// Consider moving to crate win32
mod win32;
mod app;

fn main() {
    // https://github.com/rust-lang/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    // Potential macro to handle Option<T> failures:
    // `macro_rules! try_option {($x:expr) => (match $x {Some(x) => x, None => return})}`
    // Otherwise use try! with Result<T, E>

    let create_result = DummyWindow::create(None, main_wnd_proc as WNDPROC);

    match create_result {
        Ok(_) => {
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
    match msg {
        1234 => {
            match lParam as UINT {
                WM_LBUTTONDBLCLK => {
                    DestroyWindow(hWnd);
                }
                WM_RBUTTONDOWN | WM_CONTEXTMENU => {
                    // show_popup_menu(hWnd);
                }
                _ => { }
            }
        }

        WM_COMMAND => {
            match LOWORD(wParam as DWORD) {
                1 => { // TM_EXIT
                    DestroyWindow(hWnd);
                }
                _ => { }
            }
        }

        WM_DESTROY => {
            PostQuitMessage(0);
        }

        _ => {
            return DefWindowProcW(hWnd, msg, wParam, lParam);
        }
    }

    0 as LRESULT
}