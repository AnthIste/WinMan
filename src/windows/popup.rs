use std;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use comctl32;
use kernel32;
use user32;
use gdi32;
use winapi::*;

use utils::Win32Result;
use windows::*;

const WIN_DIMENSIONS: (i32, i32) = (340, 50);
const THEME_BG_COLOR: u32 = 0x00222222;
const THEME_EDIT_COLOR: u32 = 0x00A3FFA3;
const THEME_EDIT_BG_COLOR: u32 = 0x00323232;

const MSG_NOTIFY_RETURN: u32 = 1;
const MSG_NOTIFY_ESCAPE: u32 = 2;

type PopupInstances = ::windows::InstanceMap<PopupWindow>;
unsafe impl Send for PopupInstances {}

lazy_static! {
    static ref POPUP_INSTANCES: Mutex<PopupInstances> = Mutex::new(PopupInstances::new());
}

pub struct PopupWindow {
    hwnd: HWND,
    _edit_box: EditBox,
    hbrush_primary: HBRUSH,
    hbrush_secondary: HBRUSH,
}
struct EditBox { hwnd: HWND }

impl PopupWindow {
    pub fn new() -> Win32Result<Rc<RefCell<PopupWindow>>> {
        let (w, h) = WIN_DIMENSIONS;
        let class_name: Vec<u16> = OsStr::new("WinmanPopupWindow")
            .encode_wide()
            .chain(::std::iter::once(0))
            .collect();

        let hwnd = unsafe {
            let window_class = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: winuser::CS_HREDRAW | winuser::CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: 0 as HINSTANCE,
                hIcon: 0 as HICON,
                hCursor: user32::LoadCursorW(0 as HINSTANCE, winuser::IDC_ARROW),
                hbrBackground: 0 as HBRUSH,
                lpszMenuName: 0 as LPCWSTR,
                lpszClassName: class_name.as_ptr(),
                hIconSm: 0 as HICON,
            };

            if user32::RegisterClassExW(&window_class) == 0 {
                return Err(kernel32::GetLastError());
            }

            user32::CreateWindowExW(
                0,
                class_name.as_ptr(),
                0 as LPCWSTR,
                winuser::WS_POPUP | winuser::WS_BORDER,
                0,
                0,
                w,
                h,
                0 as HWND,
                0 as HMENU,
                0 as HINSTANCE,
                0 as LPVOID)
        };

        // Creation is a 3-step process:
        //
        //   1. Create the shell HWND (above)
        //   2. Add all window children
        //   3. Register the instance in global state so that it can be referenced statically
        //
        // If we error on step 2, we must destroy the window instance before returning
        // The parent HWND is not managed and will be forgotten on an early return
        // The easiest way to ensure this is to perform the rest of the layout in a separate funcction
        let create_result = PopupWindow::new_impl(hwnd);

        match create_result {
            Ok(window) => {
                let mut map = POPUP_INSTANCES.lock().unwrap();
                let shared = map.set(window.hwnd, window);

                Ok(shared)
            },

            Err(e) => {
                unsafe { user32::DestroyWindow(hwnd); }
                Err(e)
            }
        }
    }

    fn new_impl(hwnd: HWND) -> Win32Result<PopupWindow> {
        // Create controls
        let bounds_window = get_window_bounds(hwnd);

        let edit_box = {
            let height = 22;
            let padding = (15, 0, 15, 0);
            let bounds_edit = calc_window_pos(
                bounds_window,
                None,
                Some(height),
                None,
                Some(padding),
                HorizontalAlignment::Center,
                VerticalAlignment::Center);

            try!{ EditBox::new(hwnd, bounds_edit) }
        };

        // Set initial focus
        unsafe { user32::SetFocus(edit_box.hwnd); }

        // Create brush resources
        // TODO: dispose
        let hbrush_primary = unsafe { gdi32::CreateSolidBrush(THEME_BG_COLOR) };
        let hbrush_secondary = unsafe { gdi32::CreateSolidBrush(THEME_EDIT_BG_COLOR) };
        
        Ok(PopupWindow {
            hwnd: hwnd,
            _edit_box: edit_box,
            hbrush_primary: hbrush_primary,
            hbrush_secondary: hbrush_secondary,
        })
    }

    pub fn show(&self) {
        let screen_bounds = get_screen_bounds();
        let (w, h) = WIN_DIMENSIONS;
        let (x, y, w, h) = calc_window_pos(
            screen_bounds,
            Some(w),
            Some(h),
            None,
            None,
            HorizontalAlignment::Center,
            VerticalAlignment::Center);

        unsafe {
            user32::SetWindowPos(self.hwnd, winuser::HWND_TOPMOST, x, y, w, h, 0);
            user32::ShowWindow(self.hwnd, 5); // SW_SHOW
        }
    }

    pub fn _hide(&self) {
        unsafe {
            user32::ShowWindow(self.hwnd, 0); // SW_HIDE
        }
    }

    fn wm_erasebkgnd(&self, hdc: HDC) -> Option<HBRUSH> {
        let brush = self.hbrush_primary;

        // Background
        let mut rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        unsafe {
            user32::GetClientRect(self.hwnd, &mut rc);
            user32::FillRect(hdc, &rc, brush);
        }

        // Border
        Some(self.hbrush_secondary)
    }

    fn wm_ctlcoloredit(&self, hdc: HDC) -> Option<HBRUSH> {
        unsafe {
            gdi32::SetBkColor(hdc, THEME_EDIT_BG_COLOR);
            gdi32::SetTextColor(hdc, THEME_EDIT_COLOR);
        }

        Some(self.hbrush_secondary)
    }

    fn wm_notify(&self, nmhdr: &winuser::NMHDR) {
        match nmhdr.code {
            MSG_NOTIFY_ESCAPE => {
                unsafe { user32::PostQuitMessage(0); }
            },

            MSG_NOTIFY_RETURN => {
                println!("MSG_NOTIFY_RETURN");
            },

            _ => ()
        }
    }

    fn wm_keydown(&self, vk: i32, _flags: i32) {
        match vk {
            VK_ESCAPE => {
                unsafe { user32::PostQuitMessage(0); }
            },

            _ => ()
        }
    }
}

impl EditBox {
    fn new(parent: HWND, bounds: Bounds) -> Win32Result<Self> {
        // Using Edit Controls
        // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775462(v=vs.85).aspx
        let class_name: Vec<u16> = OsStr::new("Edit")
            .encode_wide()
            .chain(::std::iter::once(0))
            .collect();

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
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let instance = {
        let map = POPUP_INSTANCES.lock().unwrap();
        map.get(hwnd)
    };

    if let Some(instance) = instance {
        match msg {
            WM_ERASEBKGND => {
                let hdc: HDC = wparam as HDC;
                let dc_brush = instance.borrow().wm_erasebkgnd(hdc);
                let dc_brush = dc_brush.unwrap_or(0 as HBRUSH);
                
                return dc_brush as LRESULT;
            },

            WM_CTLCOLOREDIT => {
                let hdc: HDC = wparam as HDC;
                let dc_brush = instance.borrow().wm_ctlcoloredit(hdc);
                let dc_brush = dc_brush.unwrap_or(0 as HBRUSH);

                return dc_brush as LRESULT;
            },

            WM_NOTIFY => {
                let nmhdr = lparam as *const winuser::NMHDR;
                instance.borrow().wm_notify(&*nmhdr);

                return 0;
            },

            WM_KEYDOWN => {
                let vk = wparam as i32;
                let flags = lparam as i32;
                instance.borrow().wm_keydown(vk, flags);

                return 0;
            },

            WM_DESTROY => {
                user32::PostQuitMessage(0);
                return 0;
            },
            
            _ => {}
        }
    };

    user32::DefWindowProcW(hwnd, msg, wparam, lparam)
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

                _ => {}
            }
        },
        
        _ => {}
    }

    comctl32::DefSubclassProc(hwnd, msg, wparam, lparam)
}