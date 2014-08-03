use win32::types::*;

pub static WM_HOTKEY: UINT = 0x0312;

pub static MOD_ALT: UINT = 0x0001;
pub static MOD_CONTROL: UINT = 0x0002;
pub static MOD_SHIFT: UINT = 0x0004;
pub static MOD_WIN: UINT = 0x0008;
pub static MOD_NOREPEAT: UINT = 0x4000;

pub static VK_0: UINT = 0x30;
pub static VK_1: UINT = 0x31;
pub static VK_2: UINT = 0x32;
pub static VK_3: UINT = 0x33;
pub static VK_4: UINT = 0x34;
pub static VK_5: UINT = 0x35;
pub static VK_6: UINT = 0x36;
pub static VK_7: UINT = 0x37;
pub static VK_8: UINT = 0x38;
pub static VK_9: UINT = 0x39;
pub static VK_Q: UINT = 0x51;