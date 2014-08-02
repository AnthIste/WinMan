use win32::types::*;

mod ffi {
    use win32::types::*;

    #[link(name = "user32")]
    extern "stdcall" {
        pub fn MessageBoxA(
            hWnd: HWND, lpText: LPCSTR, lpCaption: LPCSTR, uType: UINT
        ) -> c_int;

        pub fn MessageBoxW(
            hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT
        ) -> c_int;

        pub fn RegisterHotKey(
            hWnd: HWND, id: c_int, fsModifiers: UINT, vk: UINT
        ) -> BOOL;

        pub fn GetMessageA(
            lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT
        ) -> BOOL;

        pub fn GetMessageW(
            lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT
        ) -> BOOL;
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

pub fn RegisterHotKey(
    hWnd: HWND, id: c_int, fsModifiers: UINT, vk: UINT
) -> BOOL {
    unsafe { ffi::RegisterHotKey(hWnd, id, fsModifiers, vk) }
}

pub fn GetMessageA(
    lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT
) -> BOOL {
    unsafe { ffi::GetMessageA(lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax) }
}

pub fn GetMessageW(
    lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT
) -> BOOL {
    unsafe { ffi::GetMessageW(lpMsg, hWnd, wMsgFilterMin, wMsgFilterMax) }
}