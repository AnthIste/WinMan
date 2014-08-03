#![feature(globs)]

extern crate libc;

use std::default::Default;

use win32::constants::*;
use win32::types::{HWND, MSG, UINT, DWORD, WORD};
use win32::window::{MessageBoxA,GetMessageW,TranslateMessage,DispatchMessageW,RegisterHotKey,PostQuitMessage};

// Consider moving to crate win32
mod win32;

static MOD_APP: UINT = MOD_ALT | MOD_CONTROL;
static MOD_GRAB: UINT = MOD_ALT | MOD_SHIFT;
static MOD_SWITCH: UINT = MOD_ALT;

fn register_hotkeys() {
    // CTRL-ALT-Q to quit
    RegisterHotKey(0 as HWND, 0, MOD_APP, VK_Q);

    // ALT-SHIFT-1 to ALT-SHIFT-9 to grab windows,
    // ALT-1 to ALT-9 to switch windows
    for i in range(1, 10) {
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

fn loword(u: DWORD) -> WORD {
    ((u & 0xFFFF0000) >> 16)  as WORD
}

fn hiword(u: DWORD) -> WORD {
    (u & 0x0000FFFF) as WORD
}

fn extract_hotkey(msg: &MSG) -> (UINT, UINT) {
    let modifiers = hiword(msg.lParam as DWORD) as UINT;
    let vk = loword(msg.lParam as DWORD) as UINT;

    (modifiers, vk)
}

fn main() {
    // https://github.com/rust-lang/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    register_hotkeys();

    let mut msg: MSG = Default::default();

    while GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);

        if msg.message == WM_HOTKEY {
            let hotkey = extract_hotkey(&msg);
            process_hotkey(hotkey);
        }
    }

    MessageBoxA(0 as HWND, "All done!".to_c_str().as_ptr(), "Exiting".to_c_str().as_ptr(), 0);
}