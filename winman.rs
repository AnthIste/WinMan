#![feature(globs)]

extern crate libc;

use std::default::Default;
use std::iter::range_inclusive;

use win32::constants::*;
use win32::types::{HWND,MSG,UINT,DWORD,WORD,NOTIFYICONDATA,WNDCLASSEX,WNDPROC,WPARAM,LPARAM,LRESULT,HMENU,
                   HINSTANCE,LPVOID,LPCWSTR,ULONG_PTR,HICON,POINT,RECT};
use win32::window::{MessageBoxA,GetMessageW,TranslateMessage,DispatchMessageW,RegisterHotKey,PostQuitMessage,
                    Shell_NotifyIcon,RegisterClassExW,DefWindowProcW,CreateWindowExW,GetLastError,LoadImageW,
                    GetSystemMetrics,GetModuleHandleW,CreatePopupMenu,AppendMenuA,GetCursorPos,
                    SetForegroundWindow,TrackPopupMenu,DestroyWindow};
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

struct DummyWindow {
    pub hWnd: HWND,
    pub nid: Option<NOTIFYICONDATA>,
}

impl DummyWindow {
    pub fn create(wndProc: WNDPROC) -> Result<DummyWindow, u32> {
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
            return Err(GetLastError());
        }

        let dummy_window = DummyWindow {
            hWnd: hWnd,
            nid: None,
        };

        Ok(dummy_window)
    }

    pub fn register_systray_icon(&mut self) {
        match self.nid {
            None => {
                let hInstance = GetModuleHandleW(0 as LPCWSTR);
                let mut nid: NOTIFYICONDATA = Default::default();

                nid.uID = 0x29A;
                nid.uCallbackMessage = 1234;
                nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
                nid.hWnd = self.hWnd;
                nid.hIcon = LoadImageW(
                    hInstance,
                    MAKEINTRESOURCEW(IDI_ICON1),
                    1, // IMAGE_ICON
                    GetSystemMetrics(49), // SM_CXSMICON
                    GetSystemMetrics(50), // SM_CYSMICON
                    0 // LR_DEFAULTCOLOR
                    ) as HICON;

                Shell_NotifyIcon(NIM_ADD, &mut nid);

                self.nid = Some(nid);
            }
            Some(_) => { }
        }
    }

    pub fn deregister_systray_icon(&mut self) {
        match self.nid {
            Some(mut nid) => {
                Shell_NotifyIcon(NIM_DELETE, &mut nid);
                self.nid = None;
            }
            None => { }
        }
    }
}

trait OnWindowMessage {
    fn on_create(&mut self) -> bool { true }

    fn on_destroy(&mut self) -> bool { true }
}

impl OnWindowMessage for DummyWindow {
    fn on_create(&mut self) -> bool {
        self.register_systray_icon();
        true
    }

    fn on_destroy(&mut self) -> bool {
        PostQuitMessage(0);
        true
    }
}

// static mut s_win: Option<&DummyWindow> = None;

fn main() {
    // https://github.com/rust-lang/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    // Potential macro to handle Option<T> failures:
    // `macro_rules! try_option {($x:expr) => (match $x {Some(x) => x, None => return})}`
    // Otherwise use try! with Result<T, E>

    let create_result = DummyWindow::create(main_wnd_proc as WNDPROC);

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
        }

        _ => {
            return DefWindowProcW(hWnd, msg, wParam, lParam);
        }
    }

    0 as LRESULT
}