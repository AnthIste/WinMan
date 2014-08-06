#![feature(globs)]

extern crate libc;

use std::default::Default;
use std::iter::range_inclusive;

use win32::constants::*;
use win32::types::{HWND,MSG,UINT,DWORD,WORD,WNDPROC,WPARAM,LPARAM,LRESULT,POINT,RECT};
use win32::window::{MessageBoxA,GetMessageW,TranslateMessage,DispatchMessageW,RegisterHotKey,PostQuitMessage,
                    DefWindowProcW,CreatePopupMenu,AppendMenuA,GetCursorPos,SetForegroundWindow,TrackPopupMenu,
                    DestroyWindow};

use app::window::{Win32Result,Win32Window};
use app::dummy::{DummyWindow};

// Consider moving to crate win32
mod win32;
mod app;

static MOD_APP: UINT = MOD_ALT | MOD_CONTROL;
static MOD_GRAB: UINT = MOD_ALT | MOD_SHIFT;
static MOD_SWITCH: UINT = MOD_ALT;

fn show_popup_menu(hWnd: HWND) {
    let mut curPoint: POINT = Default::default();
    GetCursorPos(&mut curPoint);

    let hMenu = CreatePopupMenu();
    AppendMenuA(
        hMenu,
        0, // MF_STRING
        1, // TM_EXIT
        "E&xit".to_c_str().as_ptr()
        );

    SetForegroundWindow(hWnd);

    TrackPopupMenu(hMenu,
                   0x80 | 0x4 | 0x20, // TPM_NONOTIFY | TPM_CENTERALIGN | TPM_BOTTOMALIGN,
                   curPoint.x,
                   curPoint.y,
                   0,
                   hWnd,
                   0 as *mut RECT);
}

fn register_hotkeys() {
    // CTRL-ALT-Q to quit
    RegisterHotKey(0 as HWND, 0, MOD_APP, VK_Q);

    // ALT-SHIFT-1 to ALT-SHIFT-9 to grab windows,
    // ALT-1 to ALT-9 to switch windows
    for i in range_inclusive(1, 9) {
        let vk_n = VK_0 + i;

        RegisterHotKey(0 as HWND, 1, MOD_GRAB, vk_n);
        RegisterHotKey(0 as HWND, 2, MOD_SWITCH, vk_n);
    }
}

fn is_window_hotkey(vk: UINT) -> bool {
    vk >= VK_1 && vk <= VK_9
}

fn process_hotkey(hotkey: (UINT, UINT)) {
    match hotkey {
        (MOD_APP, VK_Q) => {
            PostQuitMessage(0)
        }

        (MOD_GRAB, vk) if is_window_hotkey(vk) => {
            // Grab and map foreground window
        }

        (MOD_SWITCH, vk) if is_window_hotkey(vk) => {
            // Switch to mapped window
        }

        _ => { }
    }
}

fn hiword(u: DWORD) -> WORD {
    ((u & 0xFFFF0000) >> 16) as WORD
}

fn loword(u: DWORD) -> WORD {
    (u & 0x0000FFFF) as WORD
}

fn extract_hotkey(msg: &MSG) -> (UINT, UINT) {
    let modifiers = loword(msg.lParam as DWORD) as UINT;
    let vk = hiword(msg.lParam as DWORD) as UINT;

    (modifiers, vk)
}

fn main() {
    // https://github.com/rust-lang/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    // Potential macro to handle Option<T> failures:
    // `macro_rules! try_option {($x:expr) => (match $x {Some(x) => x, None => return})}`
    // Otherwise use try! with Result<T, E>

    let create_result: Win32Result<DummyWindow> = Win32Window::create(None, main_wnd_proc as WNDPROC);

    match create_result {
        Ok(mut dummy_window) => {
            register_hotkeys();
            dummy_window.register_systray_icon();

            let mut msg: MSG = Default::default();

            while GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);

                // Hotkeys are sent to the thread, not the window
                if msg.message == WM_HOTKEY {
                    let hotkey = extract_hotkey(&msg);
                    process_hotkey(hotkey);
                }
            }

            dummy_window.deregister_systray_icon();

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
                    show_popup_menu(hWnd);
                }
                _ => { }
            }
        }

        WM_COMMAND => {
            match loword(wParam as DWORD) {
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