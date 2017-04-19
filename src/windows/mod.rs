use user32;
use winapi::windef::*;
use winapi::winuser;

pub mod messages;
pub mod popup;

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