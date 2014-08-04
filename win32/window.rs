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

        pub fn TranslateMessage(
            lpMsg: LPMSG
        ) -> BOOL;

        pub fn DispatchMessageA(
            lpMsg: LPMSG
        ) -> BOOL;

        pub fn DispatchMessageW(
            lpMsg: LPMSG
        ) -> BOOL;

        pub fn PostQuitMessage(
            nExitCode: c_int
        );

        pub fn Shell_NotifyIcon(
            dwMessage: DWORD, lpdata: PNOTIFYICONDATA
        ) -> BOOL;

        pub fn RegisterClassExW(
            lpwcx: *const WNDCLASSEX
        ) -> ATOM;

        pub fn CreateWindowExW(
            extrastyle: DWORD, classname: LPCWSTR,
            windowname: LPCWSTR, style: DWORD,
            x: c_int, y: c_int, width: c_int, height: c_int,
            parent: HWND, menu: HMENU, instance: HINSTANCE, param: LPVOID
        ) -> HWND;

        pub fn ShowWindow(hwnd: HWND, nCmdShow: c_int) -> BOOL;

        pub fn DefWindowProcW(
            hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM
        ) -> LRESULT;
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetLastError() -> DWORD;
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

pub fn TranslateMessage(
    lpMsg: LPMSG
) -> BOOL {
    unsafe { ffi::TranslateMessage(lpMsg) }
}

pub fn DispatchMessageA(
    lpMsg: LPMSG
) -> BOOL {
    unsafe { ffi::DispatchMessageA(lpMsg) }
}

pub fn DispatchMessageW(
    lpMsg: LPMSG
) -> BOOL {
    unsafe { ffi::DispatchMessageW(lpMsg) }
}

pub fn PostQuitMessage(
    nExitCode: c_int
) {
    unsafe { ffi::PostQuitMessage(nExitCode) }
}

pub fn Shell_NotifyIcon(
    dwMessage: DWORD, lpdata: PNOTIFYICONDATA
) -> BOOL {
    unsafe { ffi::Shell_NotifyIcon(dwMessage, lpdata) }
}

pub fn RegisterClassExW(
    lpwcx: *const WNDCLASSEX
) -> ATOM {
    unsafe { ffi::RegisterClassExW(lpwcx) }
}

pub fn CreateWindowExW(
    extrastyle: DWORD, classname: LPCWSTR,
    windowname: LPCWSTR, style: DWORD,
    x: c_int, y: c_int, width: c_int, height: c_int,
    parent: HWND, menu: HMENU, instance: HINSTANCE, param: LPVOID
) -> HWND {
    unsafe { ffi::CreateWindowExW(extrastyle, classname, windowname, style, x, y, width, height, parent, menu, instance, param) }
}

pub fn DefWindowProcW(
    hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM
) -> LRESULT {
    unsafe { ffi::DefWindowProcW(hwnd, msg, wparam, lparam) }
}

pub fn GetLastError() -> DWORD {
    unsafe { ffi::GetLastError() }
}
