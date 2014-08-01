use win32::types::{HWND, LPCSTR, LPCWSTR, UINT, c_int};

mod ffi {
    use win32::types::{HWND, LPCSTR, LPCWSTR, UINT, c_int};

    #[link(name = "user32")]
    extern "stdcall" {
        pub fn MessageBoxA(
            hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT
        ) -> c_int;

        pub fn MessageBoxW(
            hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
        ) -> c_int;
    }
}

pub fn MessageBoxW(
    hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
) -> c_int {
    unsafe { ffi::MessageBoxW(hWnd, lpText, lpCaption, uType) }
}

pub fn MessageBoxA(
    hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT
) -> c_int {
    unsafe { ffi::MessageBoxA(hWnd, lpText, lpCaption, uType) }
}