use win32::types::{HINSTANCE,HWND,WNDPROC};

pub type Win32Result<T> = Result<T, u32>;

pub trait Win32Window {
    fn create(hInstance: Option<HINSTANCE>, wndProc: WNDPROC) -> Win32Result<Self>;

    fn get_hwnd(&self) -> HWND;

    fn get_hinstance(&self) -> Option<HINSTANCE>;
}

pub trait OnWindowMessage {
    fn on_create(&mut self) -> bool { true }

    fn on_destroy(&mut self) -> bool { true }
}