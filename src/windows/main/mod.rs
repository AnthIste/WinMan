use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::Mutex;

use winapi::*;
use kernel32;
use user32;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use winapi::winuser::*;

use constants::*;
use utils::Win32Result;
use window_tracking::Config;
use windows::ManagedWindow2;

const CLASS_NAME: &'static str = "WinmanMainWindow";

// Hotkey modifiers (TODO: USE MK_CONTROL etc)
const MOD_APPCOMMAND: UINT = MOD_CONTROL | MOD_ALT;
const MOD_GRAB_WINDOW: UINT = MOD_ALT | MOD_SHIFT;
const MOD_SWITCH_WINDOW: UINT = MOD_ALT;
const MOD_CLEAR_WINDOWS: UINT = MOD_CONTROL | MOD_ALT;

pub enum AppMsg {
    _Hotkey
}

struct AppWindow { hwnd: HWND }

impl AppWindow {
    pub fn register_classes() -> Win32Result<()> {
        let class_name: Vec<u16> = OsStr::new(CLASS_NAME)
            .encode_wide()
            .chain(::std::iter::once(0))
            .collect();

        let mut window_class: WNDCLASSEXW = unsafe { ::std::mem::zeroed() };
        window_class.cbSize = ::std::mem::size_of::<WNDCLASSEXW>() as u32;
        window_class.lpfnWndProc = Some(AppWindow::window_proc);
        window_class.lpszClassName = class_name.as_ptr();

        unsafe {
            match user32::RegisterClassExW(&window_class) {
                0 => Err(kernel32::GetLastError()),
                _ => Ok(())
            }
        }
    }

    pub fn new() -> Win32Result<ManagedWindow2<Self>> {
        let class_name: Vec<u16> = OsStr::new(CLASS_NAME)
            .encode_wide()
            .chain(::std::iter::once(0))
            .collect();

        let hwnd = unsafe {
            let hwnd = user32::CreateWindowExW(
                0,
                class_name.as_ptr(),
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

            if hwnd == 0 as HWND {
                return Err(kernel32::GetLastError());
            }

            hwnd
        };

        let app = AppWindow {
            hwnd: hwnd
        };

        Ok(ManagedWindow2::new(hwnd, Box::new(app)).unwrap())
    }

    unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        user32::DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}
