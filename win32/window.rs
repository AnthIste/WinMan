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