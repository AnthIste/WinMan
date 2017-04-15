use std;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use kernel32;
use user32;
use winapi::*;
use winapi::windef::RECT;
use winapi::winuser;

use utils::Win32Result;

const WIN_DIMENSIONS: (i32, i32) = (340, 50);

pub struct PopupWindow {
    hwnd: HWND
}

impl PopupWindow {
    fn new(hwnd: HWND) -> Self {
        PopupWindow {
            hwnd: hwnd
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

pub fn create_window() -> Win32Result<PopupWindow> {
    let popup_window = create_window_impl(Some(window_proc))
        .and_then(|hwnd| {
            create_edit_box(hwnd).map(|_| hwnd)
        });

    match popup_window {
        Ok(hwnd) => Ok(PopupWindow::new(hwnd)),
        Err(e) => Err(e),
    }
}

fn create_window_impl(window_proc: WNDPROC) -> Win32Result<HWND> {
    let (w, h) = WIN_DIMENSIONS;
    let class_name: Vec<u16> = OsStr::new("WinmanPopupWindow").encode_wide().collect();

    let hwnd = unsafe {
        let window_class = WNDCLASSEXW {
        	cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        	style: winuser::CS_HREDRAW | winuser::CS_VREDRAW,
        	lpfnWndProc: window_proc,
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

        let hwnd = user32::CreateWindowExW(
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
            0 as LPVOID);

        if hwnd == 0 as HWND {
            return Err(kernel32::GetLastError());
        }

        hwnd
    };

    Ok(hwnd)
}

fn get_screen_bounds() -> (i32, i32, i32, i32) {
    let (screen_w, screen_h) = unsafe {
        (
            user32::GetSystemMetrics(SM_CXSCREEN),
            user32::GetSystemMetrics(SM_CYSCREEN),
        )
    };

    (0, 0, screen_w, screen_h)
}

fn get_window_bounds(hwnd: HWND) -> (i32, i32, i32, i32) {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0
    };

    unsafe {
        user32::GetWindowRect(hwnd, &mut rect as *mut _);
    }
    
    (rect.left, rect.top, rect.right, rect.bottom)
}

enum HorizontalAlignment {
    Left, Center, Right
}
enum VerticalAlignment {
    Top, Center, Bottom
}

fn calc_window_pos(
    parent: (i32, i32, i32, i32),
    width: Option<i32>,
    height: Option<i32>,
    margin: Option<(i32, i32, i32, i32)>,
    padding: Option<(i32, i32, i32, i32)>,
    hor_align: HorizontalAlignment,
    vert_align: VerticalAlignment) -> (i32, i32, i32, i32) {
    
    // Parent bounds
    let (l, t, r, b) = parent;
    let (parent_w, parent_h) = (r - l, b - t);

    // Self bounds
    let w = width.unwrap_or(parent_w);
    let h = height.unwrap_or(parent_h);
    let x = match hor_align {
        HorizontalAlignment::Left => 0,
        HorizontalAlignment::Center => (parent_w / 2) - (w / 2),
        HorizontalAlignment::Right => parent_w - w,
    };
    let y = match vert_align {
        VerticalAlignment::Top => 0,
        VerticalAlignment::Center => (parent_h / 2) - (h / 2),
        VerticalAlignment::Bottom => parent_h - h,
    };

    // Bounds modifiers (margin, padding)
    let (margin_left, margin_top, margin_right, margin_bottom) =
        margin.unwrap_or((0, 0, 0, 0));
    let (padding_left, padding_top, padding_right, padding_bottom) =
        padding.unwrap_or((0, 0, 0, 0));

    (
        x + margin_left - margin_right + padding_left,
        y + margin_top - margin_bottom + padding_top,
        w - padding_left - padding_right,
        h - padding_top - padding_bottom
    )
}

fn create_edit_box(parent: HWND) -> Win32Result<HWND> {
    let height = 22;
    let padding = (15, 0, 15, 0);
    let (x, y, w, h) = calc_window_pos(
        get_window_bounds(parent),
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
    let lresult = match msg {
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
    };

    lresult.unwrap_or_else(|| user32::DefWindowProcW(hwnd, msg, wparam, lparam))
}