use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::Mutex;

use winapi::*;
use kernel32;
use user32;
use spmc;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use winapi::winuser::*;

use constants::*;
use utils::Win32Result;
use window_tracking::Config;
use windows::ManagedWindow2;

const CLASS_NAME: &'static str = "WinmanMainWindow";

const HK_QUIT: i32 = 1;
const HK_POPUP: i32 = 2;
const HK_GRAB: i32 = 3;
const HK_SWITCH: i32 = 4;
const HK_CLEAR: i32 = 5;

const MOD_QUIT: u32 = MOD_CONTROL | MOD_ALT;
const MOD_POPUP: u32 = MOD_NOREPEAT | MOD_ALT;
const MOD_GRAB: u32 = MOD_NOREPEAT| MOD_ALT | MOD_SHIFT;
const MOD_SWITCH: u32 = MOD_NOREPEAT | MOD_ALT;
const MOD_CLEAR: u32 = MOD_NOREPEAT | MOD_CONTROL | MOD_ALT | MOD_SHIFT;

pub enum AppMsg {
    ShowPopup,
    GrabWindow(u32),
    FocusWindow(u32),
    ClearWindow(u32),
}

pub struct AppWindow {
    hwnd: HWND,
    tx: spmc::Sender<AppMsg>,
    rx: spmc::Receiver<AppMsg>,
}

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

        register_hotkeys(hwnd);

        let (tx, rx) = spmc::channel();
        let app = AppWindow {
            hwnd: hwnd,
            tx: tx,
            rx: rx,
        };

        Ok(ManagedWindow2::new(hwnd, Box::new(app)).unwrap())
    }

    pub fn listen(&self) -> spmc::Receiver<AppMsg> {
        self.rx.clone()
    }

    fn on_hotkey(&self, id: i32, _modifiers: u32, vk: u32) {
        match (id, vk) {
            (HK_QUIT, _) => {
                unsafe { user32::PostQuitMessage(0); }
            },

            (HK_POPUP, _) => {
                self.tx.send(AppMsg::ShowPopup);
            },

            (HK_GRAB, vk) => {
                self.tx.send(AppMsg::GrabWindow(vk));
            },

            (HK_SWITCH, vk) => {
                self.tx.send(AppMsg::FocusWindow(vk));
            },

            (HK_CLEAR, vk) => {
                self.tx.send(AppMsg::ClearWindow(vk));
            },

            _ => {}
        }
    }

    unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let instance = ManagedWindow2::<AppWindow>::get_instance_mut(hwnd);

        if let Some(instance) = instance {
            match msg {
                WM_HOTKEY => {
                    let id = wparam as i32;
                    let modifiers = LOWORD(lparam as DWORD) as u32;
                    let vk = HIWORD(lparam as DWORD) as u32;
                    instance.on_hotkey(id, modifiers, vk);

                    return 0;
                },

                WM_DESTROY => {
                    user32::PostQuitMessage(0);
                    return 0;
                },

                _ => {}
            }
        }

        user32::DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

fn register_hotkeys(hwnd: HWND) {
    // Virtual key codes: https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
    // CTRL-ALT-Q to quit
    unsafe {
        user32::RegisterHotKey(hwnd, HK_QUIT, MOD_QUIT, VK_Q);
        user32::RegisterHotKey(hwnd, HK_POPUP, MOD_POPUP, 0x20); // VK_SPACE
    }

    // ALT-SHIFT-1 to ALT-SHIFT-9 to grab windows,
    // ALT-1 to ALT-9 to switch windows
    for i in 0..10 {
        let vk_n = VK_0 + i;

        unsafe {
            user32::RegisterHotKey(hwnd, HK_GRAB, MOD_GRAB, vk_n);
            user32::RegisterHotKey(hwnd, HK_SWITCH, MOD_SWITCH, vk_n);
            user32::RegisterHotKey(hwnd, HK_CLEAR, MOD_CLEAR, vk_n);
        }
    }
}