use win32::types::*;

pub const SW_HIDE: c_int = 0;
pub const SW_MAXIMIZE: c_int = 3;
pub const SW_SHOWMAXIMIZED: c_int = 3;
pub const SW_SHOW: c_int = 5;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_RESTORE: c_int = 9;

pub const WM_CREATE: UINT = 0x0001;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_COMMAND: UINT = 0x0111;
pub const WM_CONTEXTMENU: UINT = 0x007B;
pub const WM_LBUTTONDBLCLK: UINT = 0x0203;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const WM_HOTKEY: UINT = 0x0312;
pub const WM_USER: UINT = 0x0400;

pub const MOD_ALT: UINT = 0x0001;
pub const MOD_CONTROL: UINT = 0x0002;
pub const MOD_SHIFT: UINT = 0x0004;
pub const MOD_WIN: UINT = 0x0008;
pub const MOD_NOREPEAT: UINT = 0x4000;

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

pub const NIM_ADD: DWORD = 0x00000000;
pub const NIM_MODIFY: DWORD = 0x00000001;
pub const NIM_DELETE: DWORD = 0x00000002;
pub const NIM_SETFOCUS: DWORD = 0x00000003;
pub const NIM_SETVERSION: DWORD = 0x00000004;

pub const NIF_MESSAGE: UINT = 0x00000001;
pub const NIF_ICON: UINT = 0x00000002;
pub const NIF_TIP: UINT = 0x00000004;
pub const NIF_STATE: UINT = 0x00000008;
pub const NIF_INFO: UINT = 0x00000010;
pub const NIF_GUID: UINT = 0x00000020;
pub const NIF_REALTIME: UINT = 0x00000040;
pub const NIF_SHOWTIP: UINT = 0x00000080;
