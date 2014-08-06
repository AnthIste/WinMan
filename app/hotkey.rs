use std::iter::range_inclusive;

use win32::constants::*;
use win32::types::{UINT,HWND};
use win32::window::{RegisterHotKey,PostQuitMessage};

static MOD_APP: UINT = MOD_ALT | MOD_CONTROL;
static MOD_GRAB: UINT = MOD_ALT | MOD_SHIFT;
static MOD_SWITCH: UINT = MOD_ALT;

// TODO: store configs and grab state
pub struct HotkeyManager;

impl HotkeyManager {
    pub fn new() -> HotkeyManager {
        HotkeyManager
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

    pub fn process_hotkey(&self, hotkey: (UINT, UINT)) {
        match hotkey {
            (MOD_APP, VK_Q) => {
                PostQuitMessage(0)
            }

            (MOD_GRAB, vk) if HotkeyManager::is_window_hotkey(vk) => {
                // Grab and map foreground window
            }

            (MOD_SWITCH, vk) if HotkeyManager::is_window_hotkey(vk) => {
                // Switch to mapped window
            }

            _ => { }
        }
    }

    fn is_window_hotkey(vk: UINT) -> bool {
        vk >= VK_1 && vk <= VK_9
    }
}