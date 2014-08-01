extern crate libc;

use win32::types::HWND;
use win32::wstr::ToCWStr;
use win32::window::{MessageBoxA,MessageBoxW};

// Consider moving to crate win32
mod win32;

fn main() {
    MessageBoxA(0 as HWND, "text (cstr)".to_c_str().as_ptr(), "title (cstr)".to_c_str().as_ptr(), 0);
    MessageBoxW(0 as HWND, "text (wcstr)".to_c_wstr().as_ptr(), "title (wcstr)".to_c_wstr().as_ptr(), 0);
}