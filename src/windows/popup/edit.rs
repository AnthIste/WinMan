use comctl32;
use kernel32;
use user32;
use winapi::*;

use utils;
use utils::Win32Result;
use windows::*;

const MSG_NOTIFY_RETURN: u32 = 1;
const MSG_NOTIFY_ESCAPE: u32 = 2;
const MSG_NOTIFY_CHAR: u32 = 3;

pub struct EditBox { pub hwnd: HWND }

impl EditBox {
    pub fn new(parent: HWND, bounds: Bounds) -> Win32Result<Self> {
        // Using Edit Controls
        // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775462(v=vs.85).aspx
        let class_name = utils::to_wide_chars("Edit");

        let (x, y, w, h) = bounds;
        let hwnd = unsafe {
            let hwnd = user32::CreateWindowExW(
                0, //winuser::WS_EX_CLIENTEDGE,
                class_name.as_ptr(),
                0 as LPCWSTR,
                winuser::WS_VISIBLE
                    | winuser::WS_CHILD
                    | winuser::ES_MULTILINE
                    | winuser::ES_LEFT | winuser::ES_AUTOHSCROLL | ES_AUTOVSCROLL,
                x,
                y,
                w,
                h,
                parent,
                0 as HMENU,
                0 as HINSTANCE,
                0 as LPVOID);
            
            if hwnd == 0 as HWND {
                return Err(kernel32::GetLastError());
            }

            hwnd
        };
        // Apply inner padding
        // The size cannot be too small or it will not take effect
        let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        unsafe {
            user32::SendMessageW(hwnd, EM_GETRECT as UINT, 0, (&rect as *const _) as LPARAM);
            rect.left += 5;
            rect.top += 2;
            rect.bottom += 2;
            user32::SendMessageW(hwnd, EM_SETRECT as UINT, 0, (&rect as *const _) as LPARAM);
        }

        // Subclass the window proc to allow message intercepting
        unsafe {
            comctl32::SetWindowSubclass(hwnd, Some(subclass_proc_edit), 666, 0);
        }

        Ok(EditBox {
            hwnd: hwnd
        })
    }

    pub fn get_text(&self) -> Option<String> {
        let text = unsafe {
            const BUFFER_LEN: usize = 1024;
            let buffer = [0u16; BUFFER_LEN];

            user32::SendMessageW(self.hwnd, WM_GETTEXT, BUFFER_LEN as WPARAM, buffer.as_ptr() as LPARAM);

            utils::from_wide_slice(&buffer)
        };

        if text.len() > 0 {
            Some(text)
        } else {
            None
        }
    }

    pub fn clear(&self) {
        unsafe {
            user32::SendMessageW(self.hwnd, WM_SETTEXT, 0, 0);
        }
    }
}

unsafe extern "system" fn subclass_proc_edit(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM, _: UINT_PTR, _: DWORD_PTR) -> LRESULT {
    let notify_parent = |code: u32| {
        let hwnd_parent = user32::GetParent(hwnd);
        let nmhdr = winuser::NMHDR {
            hwndFrom: hwnd,
            idFrom: 0,
            code: code,
        };
        user32::SendMessageW(hwnd_parent, WM_NOTIFY, 0 as WPARAM, (&nmhdr as *const _) as LPARAM);
    };

    match msg {
        WM_CHAR => {
            match wparam as i32 {
                VK_ESCAPE => {
                    notify_parent(MSG_NOTIFY_ESCAPE);
                    return 0;
                },

                VK_RETURN => {
                    notify_parent(MSG_NOTIFY_RETURN);
                    return 0;
                },

                _ => {
                    comctl32::DefSubclassProc(hwnd, msg, wparam, lparam);
                    notify_parent(MSG_NOTIFY_CHAR);
                    return 0;
                }
            }
        },
        
        _ => {}
    }

    comctl32::DefSubclassProc(hwnd, msg, wparam, lparam)
}