use std::collections::HashMap;
use std::iter::range_inclusive;

use win32::constants::*;
use win32::types::{UINT,HWND};
use win32::window::{RegisterHotKey,PostQuitMessage,GetForegroundWindow,SetForegroundWindow,ShowWindow};

const MOD_APP: UINT = MOD_ALT | MOD_CONTROL;
const MOD_GRAB: UINT = MOD_ALT | MOD_SHIFT;
const MOD_SWITCH: UINT = MOD_ALT;

// TODO: store configs and grab state
pub struct HotkeyManager {
    mapped_windows: HashMap<UINT, HWND>,
}

impl HotkeyManager {
    pub fn new() -> HotkeyManager {
        HotkeyManager {
            mapped_windows: HashMap::new(),
        }
    }

    pub fn register_hotkeys(&self) {
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

    pub fn process_hotkey(&mut self, hotkey: (UINT, UINT)) {
        match hotkey {
            (MOD_APP, VK_Q)  => PostQuitMessage(0),
            (MOD_GRAB, vk)   => self.grab_window(vk),
            (MOD_SWITCH, vk) => self.switch_to_window(vk),
            _                => { }
        }
    }

    fn grab_window(&mut self, vk: UINT) {
        let foreground_window = GetForegroundWindow();

        if foreground_window != 0 as HWND {
            self.mapped_windows.insert(vk, foreground_window);
        }
    }

    fn switch_to_window(&self, vk: UINT) {
        match self.mapped_windows.get(&vk) {
            Some(&hWnd) => {
                ShowWindow(hWnd, SW_SHOW);
                SetForegroundWindow(hWnd);
            }
            None => { }
        }
    }
}