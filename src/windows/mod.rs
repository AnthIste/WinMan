pub mod popup;

use user32;
use kernel32;
use winapi::windef::*;
use winapi::winuser;
use winapi::*;

use utils::Win32Result;

pub struct ManagedWindow2<T> {
    hwnd: HWND,
    data: Box<T>
}

impl<T> ManagedWindow2<T> {
    pub fn new(hwnd: HWND, data: Box<T>) -> Win32Result<Self> {
        unsafe {
            kernel32::SetLastError(0);
            let prev_value = user32::SetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA, (data.as_ref() as *const T) as LONG_PTR);

            if prev_value == 0 {
                let err = kernel32::GetLastError();
                if err != 0 {
                    return Err(err)
                }
            }
        }

        println!("Window {:?} is managed", hwnd);

        Ok(ManagedWindow2 {
            hwnd: hwnd,
            data: data
        })
    }


    pub unsafe fn get_instance_mut<'a>(hwnd: HWND) -> Option<&'a mut T> {
        let ptr = user32::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA) as *mut T;

        if !ptr.is_null() {
            Some(&mut *ptr)
        } else {
            None
        }
    }
}

impl<T> Drop for ManagedWindow2<T> {
    fn drop(&mut self) {
        unsafe { user32::SetWindowLongPtrW(self.hwnd, GWLP_USERDATA, 0); }
        println!("Window {:?} is kill", self.hwnd);
    }
}

impl<T> ::std::ops::Deref for ManagedWindow2<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data.deref()
    }
}

impl<T> ::std::ops::DerefMut for ManagedWindow2<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data.deref_mut()
    }
}

pub type Bounds = (i32, i32, i32, i32);

#[allow(dead_code)]
pub enum HorizontalAlignment {
    Left, Center, Right
}

#[allow(dead_code)]
pub enum VerticalAlignment {
    Top, Center, Bottom
}

pub fn get_screen_bounds() -> Bounds {
    let (screen_w, screen_h) = unsafe {
        (
            user32::GetSystemMetrics(winuser::SM_CXSCREEN),
            user32::GetSystemMetrics(winuser::SM_CYSCREEN),
        )
    };

    (0, 0, screen_w, screen_h)
}

pub fn get_window_bounds(hwnd: HWND) -> Bounds {
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

pub fn calc_window_pos(
    parent: Bounds,
    width: Option<i32>,
    height: Option<i32>,
    margin: Option<Bounds>,
    padding: Option<Bounds>,
    hor_align: HorizontalAlignment,
    vert_align: VerticalAlignment) -> Bounds {
    
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