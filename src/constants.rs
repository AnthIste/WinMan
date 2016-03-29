#![allow(dead_code)]

use winapi::minwindef::*;

// Key modifiers for RegisterHotKey
// https://msdn.microsoft.com/en-us/library/windows/desktop/ms646309(v=vs.85).aspx

pub const MOD_ALT: UINT = 0x0001;
pub const MOD_CONTROL: UINT = 0x0002;
pub const MOD_NOREPEAT: UINT = 0x4000;
pub const MOD_SHIFT: UINT = 0x0004;
pub const MOD_WIN: UINT = 0x0008;

// Virtual key codes:
// https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx

pub const VK_0: UINT = 0x30;
pub const VK_1: UINT = 0x31;
pub const VK_2: UINT = 0x32;
pub const VK_3: UINT = 0x33;
pub const VK_4: UINT = 0x34;
pub const VK_5: UINT = 0x35;
pub const VK_6: UINT = 0x36;
pub const VK_7: UINT = 0x37;
pub const VK_8: UINT = 0x38;
pub const VK_9: UINT = 0x39;
pub const VK_Q: UINT = 0x51;