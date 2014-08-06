use win32::types::{HINSTANCE,HWND,UINT,WORD,WPARAM,LPARAM};

pub type Win32Result<T> = Result<T, u32>;

#[allow(unused_variable)]
pub trait Win32Window {
    fn get_hwnd(&self) -> HWND;

    fn get_hinstance(&self) -> HINSTANCE;

    fn on_create(&mut self) -> Option<bool> { None }

    fn on_destroy(&mut self) -> Option<bool> { None }

    fn on_command(&mut self, command: WORD) -> Option<bool> { None }

    fn on_user(&mut self, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> Option<bool> { None }
}