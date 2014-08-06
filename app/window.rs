use win32::types::{HINSTANCE,HWND};

pub type Win32Result<T> = Result<T, u32>;

pub trait Win32Window {
    fn get_hwnd(&self) -> HWND;

    fn get_hinstance(&self) -> Option<HINSTANCE>;

    fn on_create(&mut self) -> bool { true }

    fn on_destroy(&mut self) -> bool { true }
}