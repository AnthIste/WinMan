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

// Allow mapping between static WndProc and window instance
static mut s_dummy_window: Option<DummyWindow> = None;

fn main() {
    // https://github.com/rust-lang/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    // Potential macro to handle Option<T> failures:
    // `macro_rules! try_option {($x:expr) => (match $x {Some(x) => x, None => return})}`
    // Otherwise use try! with Result<T, E>

    let dummy_window = DummyWindow::create(None, main_wnd_proc as WNDPROC);

    match dummy_window {
        Ok(window) => {
            run_application(window);

            // Signal exit
            let msg = "All done!".to_c_str();
            let title = "Exiting".to_c_str();

            MessageBoxA(0 as HWND, msg.as_ptr(), title.as_ptr(), 0);
        }
        Err(code) => {
            let msg = format!("We couldn't create a window becase of {} :<", code).to_c_str();
            let title = "Exiting".to_c_str();

            MessageBoxA(0 as HWND, msg.as_ptr(), title.as_ptr(), 0);
        }
    }
}

fn run_application(window: DummyWindow) {
    let mut hotkey_manager = HotkeyManager::new();
    let mut msg: MSG = Default::default();

    hotkey_manager.register_hotkeys();
    unsafe { s_dummy_window = Some(window); }

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
}

extern "system" fn main_wnd_proc(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        let handled =
        s_dummy_window.and_then(|mut window| match msg {
            WM_CREATE               => window.on_create(),
            WM_DESTROY              => window.on_destroy(),
            WM_COMMAND              => window.on_command(LOWORD(wParam as DWORD)),
            user if user >= WM_USER => window.on_user(msg, wParam, lParam),
            _                       => { None }
        });

        handled.unwrap_or(DefWindowProcW(hWnd, msg, wParam, lParam))
    }
}