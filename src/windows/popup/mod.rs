use winapi::*;
use kernel32;
use user32;
use gdi32;
use spmc;

use utils;
use utils::Win32Result;
use windows::*;

use self::edit::EditBox;

mod edit;

const CLASS_NAME: &'static str = "WinmanPopupWindow";

const WIN_DIMENSIONS: (i32, i32) = (340, 50);
const THEME_BG_COLOR: u32 = 0x00222222;
const THEME_EDIT_COLOR: u32 = 0x00A3FFA3;
const THEME_EDIT_BG_COLOR: u32 = 0x00323232;

const MSG_NOTIFY_RETURN: u32 = 1;
const MSG_NOTIFY_ESCAPE: u32 = 2;
const MSG_NOTIFY_CHAR: u32 = 3;

pub enum PopupMsg {
    Search(Option<String>),
    Accept(String),
}

pub struct PopupWindow {
    hwnd: HWND,
    edit_box: EditBox,
    hbrush_primary: HBRUSH,
    hbrush_secondary: HBRUSH,
    tx: spmc::Sender<PopupMsg>,
    rx: spmc::Receiver<PopupMsg>,
}

impl PopupWindow {
    pub fn register_classes() -> Win32Result<()> {
        let class_name = utils::to_wide_chars(CLASS_NAME);

        let mut window_class: WNDCLASSEXW = unsafe { ::std::mem::zeroed() };
        window_class.cbSize = ::std::mem::size_of::<WNDCLASSEXW>() as u32;
        window_class.lpfnWndProc = Some(PopupWindow::window_proc);
        window_class.lpszClassName = class_name.as_ptr();
        window_class.style = winuser::CS_HREDRAW | winuser::CS_VREDRAW;
        window_class.hCursor = unsafe { user32::LoadCursorW(0 as HINSTANCE, winuser::IDC_ARROW) };

        unsafe {
            match user32::RegisterClassExW(&window_class) {
                0 => Err(kernel32::GetLastError()),
                _ => Ok(())
            }
        }
    }

    pub fn new(hwnd_parent: HWND) -> Win32Result<ManagedWindow2<PopupWindow>> {
        let (w, h) = WIN_DIMENSIONS;
        let class_name = utils::to_wide_chars(CLASS_NAME);

        let hwnd = unsafe {
            user32::CreateWindowExW(
                0,
                class_name.as_ptr(),
                0 as LPCWSTR,
                winuser::WS_POPUP | winuser::WS_BORDER,
                0,
                0,
                w,
                h,
                hwnd_parent,
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
                ManagedWindow2::new(hwnd, Box::new(window))
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

        // Create brush resources
        // TODO: dispose
        let hbrush_primary = unsafe { gdi32::CreateSolidBrush(THEME_BG_COLOR) };
        let hbrush_secondary = unsafe { gdi32::CreateSolidBrush(THEME_EDIT_BG_COLOR) };

        // Open a channel to broadcast UI events
        let (tx, rx) = spmc::channel();

        Ok(PopupWindow {
            hwnd: hwnd,
            edit_box: edit_box,
            hbrush_primary: hbrush_primary,
            hbrush_secondary: hbrush_secondary,
            tx: tx,
            rx: rx,
        })
    }

    pub fn listen(&self) -> spmc::Receiver<PopupMsg> {
        self.rx.clone()
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
            user32::ShowWindow(self.hwnd, SW_SHOWNORMAL);
            user32::SetForegroundWindow(self.hwnd);
            user32::SetFocus(self.edit_box.hwnd);
        }

        self.edit_box.clear();
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
                self._hide();
            },

            MSG_NOTIFY_RETURN => {
                if let Some(query) = self.edit_box.get_text() {
                    let _ = self.tx.send(PopupMsg::Accept(query));
                    self.edit_box.clear();
                }
            },

            MSG_NOTIFY_CHAR => {
                let query = self.edit_box.get_text();
                let _ = self.tx.send(PopupMsg::Search(query));
            },

            _ => ()
        }
    }

    fn wm_keydown(&self, vk: i32, _flags: i32) {
        match vk {
            VK_ESCAPE => {
                self._hide();
            },

            _ => ()
        }
    }

    unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        let instance = ManagedWindow2::<PopupWindow>::get_instance_mut(hwnd);

        if let Some(instance) = instance {
            match msg {
                WM_ERASEBKGND => {
                    let hdc: HDC = wparam as HDC;
                    let dc_brush = instance.wm_erasebkgnd(hdc);
                    let dc_brush = dc_brush.unwrap_or(0 as HBRUSH);
                    
                    return dc_brush as LRESULT;
                },

                WM_CTLCOLOREDIT => {
                    let hdc: HDC = wparam as HDC;
                    let dc_brush = instance.wm_ctlcoloredit(hdc);
                    let dc_brush = dc_brush.unwrap_or(0 as HBRUSH);

                    return dc_brush as LRESULT;
                },

                WM_NOTIFY => {
                    let nmhdr = lparam as *const winuser::NMHDR;
                    instance.wm_notify(&*nmhdr);

                    return 0;
                },

                WM_KEYDOWN => {
                    let vk = wparam as i32;
                    let flags = lparam as i32;
                    instance.wm_keydown(vk, flags);

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
}