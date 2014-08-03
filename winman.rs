#![feature(globs)]

extern crate libc;

use std::default::Default;

use win32::types::{HWND, MSG, UINT};
use win32::wstr::ToCWStr;
use win32::window::{MessageBoxW,GetMessageW,TranslateMessage,DispatchMessageW,RegisterHotKey};

// Consider moving to crate win32
mod win32;

static WM_HOTKEY: UINT = 0x0312;
static MOD_ALT: UINT = 0x0001;
static MOD_CONTROL: UINT = 0x0002;
static VK_Q: UINT = 0x51;

fn main() {
    // https://github.com/rust-lang/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    // CTRL-ALT-Q to quit
    RegisterHotKey(0 as HWND, 0x29A, MOD_ALT | MOD_CONTROL, VK_Q);

    let mut msg: MSG = Default::default();
    let mut done = false;

    while !done && GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
        TranslateMessage(&mut msg);
        DispatchMessageW(&mut msg);

        if msg.message == WM_HOTKEY {
            done = true;
        }
    }

    MessageBoxW(0 as HWND, "All done!".to_c_wstr().as_ptr(), "Exiting".to_c_wstr().as_ptr(), 0);
}