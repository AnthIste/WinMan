#![feature(globs)]

extern crate libc;

use std::default::Default;
use std::iter::range_inclusive;

use win32::constants::*;
use win32::types::{HWND,MSG,UINT,DWORD,WORD,NOTIFYICONDATA,WNDCLASSEX,WNDPROC,WPARAM,LPARAM,LRESULT,HMENU,
                   HINSTANCE,LPVOID,LPCWSTR,ULONG_PTR,HICON};
use win32::window::{MessageBoxA,GetMessageW,TranslateMessage,DispatchMessageW,RegisterHotKey,PostQuitMessage,
                    Shell_NotifyIcon,RegisterClassExW,DefWindowProcW,CreateWindowExW,GetLastError,LoadImageW,
                    GetSystemMetrics,GetModuleHandleW};
use win32::wstr::ToCWStr;

// Consider moving to crate win32
mod win32;

// resource.h
static IDI_ICON1: UINT = 103;

static MOD_APP: UINT = MOD_ALT | MOD_CONTROL;
static MOD_GRAB: UINT = MOD_ALT | MOD_SHIFT;
static MOD_SWITCH: UINT = MOD_ALT;

#[allow(non_snake_case_functions)]
fn MAKEINTRESOURCEW(i: UINT) -> LPCWSTR {
    ((i as WORD) as ULONG_PTR) as LPCWSTR
}

fn create_dummy_window(wndProc: WNDPROC) -> Result<HWND, DWORD> {
    let mut wc: WNDCLASSEX = Default::default();

    wc.lpfnWndProc = wndProc;
    wc.lpszClassName = "MyMagicClassName".to_c_wstr().as_ptr();

    if RegisterClassExW(&wc) == 0 {
        return Err(GetLastError());
    }

    let hWnd = CreateWindowExW(
        0,
        "MyMagicClassName".to_c_wstr().as_ptr(),
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

    if hWnd == 0 as HWND {
        Err(GetLastError())
    }
    else {
        Ok(hWnd)
    }
}

fn register_systray_icon(hWnd: HWND) -> NOTIFYICONDATA {
    let hInstance = GetModuleHandleW(0 as LPCWSTR);
    let mut nid: NOTIFYICONDATA = Default::default();

    nid.uID = 0x29A;
    nid.uCallbackMessage = 1234;
    nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
    nid.hWnd = hWnd;
    nid.hIcon = LoadImageW(
        hInstance,
        MAKEINTRESOURCEW(IDI_ICON1),
        1, // IMAGE_ICON
        GetSystemMetrics(49), // SM_CXSMICON
        GetSystemMetrics(50), // SM_CYSMICON
        0 // LR_DEFAULTCOLOR
        ) as HICON;

    Shell_NotifyIcon(NIM_ADD, &mut nid);

    nid
}

fn deregister_systray_icon(nid: &mut NOTIFYICONDATA) {
    Shell_NotifyIcon(NIM_DELETE, nid);
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

fn loword(u: DWORD) -> WORD {
    ((u & 0xFFFF0000) >> 16) as WORD
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

    let hWndResult = create_dummy_window(main_wnd_proc as WNDPROC);

    // Potential macro to handle Option<T> failures:
    // `macro_rules! try_option {($x:expr) => (match $x {Some(x) => x, None => return})}`
    // Otherwise use try! with Result<T, E>

    match hWndResult {
        Ok(hWnd) => {
            register_hotkeys();

            let mut nid = register_systray_icon(hWnd);
            let mut msg: MSG = Default::default();

            while GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);

                if msg.message == WM_HOTKEY {
                    let hotkey = extract_hotkey(&msg);
                    process_hotkey(hotkey);
                }
            }

            deregister_systray_icon(&mut nid);

            MessageBoxA(0 as HWND, "All done!".to_c_str().as_ptr(), "Exiting".to_c_str().as_ptr(), 0);
        }

        Err(code) => {
            MessageBoxA(0 as HWND, format!("We couldn't create a window becase of {:X} :<", code).to_c_str().as_ptr(), "Exiting".to_c_str().as_ptr(), 0);
        }
    }
}

extern "system" fn main_wnd_proc(hWnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    DefWindowProcW(hWnd, msg, wParam, lParam)
}