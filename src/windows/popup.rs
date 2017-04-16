use std;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use kernel32;
use user32;
use gdi32;
use winapi::*;
use winapi::winuser;

use utils::Win32Result;
use windows::*;

const WIN_DIMENSIONS: (i32, i32) = (340, 50);
const THEME_BG_COLOR: u32 = 0x00111111;
const THEME_EDT_COLOR: u32 = 0x00A3FFA3;
const THEME_EDT_BG_COLOR: u32 = 0x00323232;

lazy_static! {
    static ref WND_MAP: Mutex<InstanceMap> = Mutex::new(InstanceMap::new());
}

type PopupWindowShared = Rc<RefCell<PopupWindow>>;

pub struct InstanceMap {
    map: HashMap<u32, PopupWindowShared>,
    err: Option<u32>,
}
unsafe impl Send for InstanceMap {}

impl InstanceMap {
    fn new() -> Self {
        InstanceMap {
            map: HashMap::new(),
            err: None,
        }
    }

    fn set(&mut self, hwnd: HWND, result: Win32Result<PopupWindow>) {
        match result {
            Ok(instance) => {
                let key = hwnd as u32;
                let shared = Rc::new(RefCell::new(instance));
        
                self.map.insert(key, shared);
            },

            Err(e) => {
                self.err = Some(e);
            }
        }
    }

    fn get(&self, hwnd: HWND) -> Option<Win32Result<PopupWindowShared>> {
        let key = hwnd as u32;

        if hwnd == 0 as HWND {
            self.err.map(|e| Err(e))
        } else {
            self.map.get(&key).map(|rc| Ok(rc.clone()))
        }
    }
}

pub struct PopupWindow {
    hwnd: HWND,
    hwnd_edit: HWND,
    hbrush_edt: HBRUSH,
}

impl PopupWindow {
    fn new(
        hwnd: HWND,
        hwnd_edit: HWND,
        hbrush_edt: HBRUSH) -> Self {
        PopupWindow {
            hwnd: hwnd,
            hwnd_edit: hwnd_edit,
            hbrush_edt: hbrush_edt,
        }
    }

    pub fn show(&mut self) {
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
            user32::SetWindowPos(self.hwnd, 0 as HWND, x, y, w, h, 0);
            user32::ShowWindow(self.hwnd, 5); // SW_SHOW
        }
    }

    pub fn hide(&mut self) {
        unsafe {
            user32::ShowWindow(self.hwnd, 0); // SW_HIDE
        }
    }
}

pub fn create_window() -> Win32Result<PopupWindowShared> {
    // TODO: dispose brush (https://msdn.microsoft.com/en-us/library/windows/desktop/dd183518(v=vs.85).aspx)
    // Wrap in drop handle? This is a global resource used in the window class
    let hbrush_bg = unsafe { gdi32::CreateSolidBrush(THEME_BG_COLOR) };

    let (w, h) = WIN_DIMENSIONS;
    let class_name: Vec<u16> = OsStr::new("WinmanPopupWindow").encode_wide().collect();

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
        	hbrBackground: hbrush_bg,
        	lpszMenuName: 0 as LPCWSTR,
        	lpszClassName: class_name.as_ptr(),
        	hIconSm: 0 as HICON,
        };

        if user32::RegisterClassExW(&window_class) == 0 {
            return Err(kernel32::GetLastError());
        }

        user32::CreateWindowExW(
            winuser::WS_EX_TOPMOST, // always on top
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

    let ref mut map = WND_MAP.lock().unwrap();
    
    map.get(hwnd).expect("Window was just created and should exist")
}

fn create_window_layout(hwnd: HWND) -> Win32Result<PopupWindow> {
    let window_bounds = get_window_bounds(hwnd);
    let hbrush_edt = unsafe { gdi32::CreateSolidBrush(THEME_EDT_BG_COLOR) };
    let hwnd_edt = try!{ create_edit_box(hwnd, window_bounds) };
    
    Ok(PopupWindow::new(
        hwnd,
        hwnd_edt,
        hbrush_edt))
}

fn create_edit_box(parent: HWND, bounds: Bounds) -> Win32Result<HWND> {
    let height = 22;
    let padding = (15, 0, 15, 0);
    let (x, y, w, h) = calc_window_pos(
        bounds,
        None,
        Some(height),
        None,
        Some(padding),
        HorizontalAlignment::Center,
        VerticalAlignment::Center);

    // Using Edit Controls
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775462(v=vs.85).aspx
    let class_name: Vec<u16> = OsStr::new("Edit")
        .encode_wide()
        .chain(::std::iter::once(0))
        .collect();

    let hwnd = unsafe {
        let hwnd = user32::CreateWindowExW(
            winuser::WS_EX_CLIENTEDGE, // The window has a border with a sunken edge
            class_name.as_ptr(),
            0 as LPCWSTR,
            winuser::WS_VISIBLE | winuser::WS_CHILD | winuser::ES_LEFT | winuser::ES_AUTOHSCROLL,
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

    Ok(hwnd)
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let lresult = WND_MAP.try_lock().ok().and_then(|mut map| {
        let instance = map.get(hwnd)
            .and_then(|r| r.ok());

        match instance {
            // Window exists
            Some(instance) => match msg {
                WM_CTLCOLOREDIT => {
                    let hdc: HDC = wparam as HDC;
                    gdi32::SetBkColor(hdc, THEME_EDT_BG_COLOR);
                    gdi32::SetTextColor(hdc, THEME_EDT_COLOR);

                    let dc_brush = instance.borrow().hbrush_edt;
                    Some(dc_brush as LPARAM)
                },

                WM_HOTKEY => {
                    let _modifiers = LOWORD(lparam as DWORD);
                    let _vk = HIWORD(lparam as DWORD);

                    Some(0)
                },

                WM_COMMAND => {
                    let _command = LOWORD(wparam as DWORD);

                    Some(0)
                },

                WM_DESTROY => {
                    user32::PostQuitMessage(0);
                    Some(0)
                },
                
                user if user >= WM_USER => Some(0),
                
                _ => None
            },

            // Window creating
            None if msg == WM_CREATE => {
                let instance = create_window_layout(hwnd);
                let lresult = match instance {
                    Err(_) => -1,
                    _ => 0,
                };

                map.set(hwnd, instance);

                Some(lresult)
            },

            // Unknown window lifecycle
            _ => None
        }
    });

    lresult.unwrap_or_else(|| user32::DefWindowProcW(hwnd, msg, wparam, lparam))
}