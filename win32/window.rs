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

        pub fn LoadImageA(
            hInst: HINSTANCE, name: LPCSTR, u_type: UINT, cx: c_int, cy: c_int, fuLoad: UINT
        ) -> HANDLE;

        pub fn LoadImageW(
            hInst: HINSTANCE, name: LPCWSTR, u_type: UINT, cx: c_int, cy: c_int, fuLoad: UINT
        ) -> HANDLE;

        pub fn GetSystemMetrics(
            nIndex: c_int
        ) -> c_int;

        pub fn CreatePopupMenu() -> HMENU;

        pub fn AppendMenuA(
            hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT, lpNewItem: LPCSTR
        ) -> BOOL;

        pub fn AppendMenuW(
            hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT, lpNewItem: LPCWSTR
        ) -> BOOL;

        pub fn GetCursorPos(
            lpPoint: LPPOINT
        ) -> BOOL;

        pub fn SetForegroundWindow(
            hWnd: HWND
        ) -> BOOL;

        pub fn TrackPopupMenu(
            hMenu: HMENU, uFlags: UINT, x: c_int, y: c_int, nReserved: c_int, hWnd: HWND, prcRect: *mut RECT
        ) -> BOOL;

        pub fn DestroyWindow(
            hWnd: HWND
        ) -> BOOL;
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetLastError() -> DWORD;

        pub fn GetModuleHandleA(
            lpModuleName: LPCSTR
        ) -> HMODULE;

        pub fn GetModuleHandleW(
            lpModuleName: LPCWSTR
        ) -> HMODULE;
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

pub fn LoadImageA(
    hInst: HINSTANCE, name: LPCSTR, u_type: UINT, cx: c_int, cy: c_int, fuLoad: UINT
) -> HANDLE {
    unsafe { ffi::LoadImageA(hInst, name, u_type, cx, cy, fuLoad) }
}

pub fn LoadImageW(
    hInst: HINSTANCE, name: LPCWSTR, u_type: UINT, cx: c_int, cy: c_int, fuLoad: UINT
) -> HANDLE {
    unsafe { ffi::LoadImageW(hInst, name, u_type, cx, cy, fuLoad) }
}

pub fn GetSystemMetrics(
    nIndex: c_int
) -> c_int {
    unsafe { ffi::GetSystemMetrics(nIndex) }
}

pub fn GetModuleHandleA(
    lpModuleName: LPCSTR
) -> HMODULE {
    unsafe { ffi::GetModuleHandleA(lpModuleName) }
}

pub fn GetModuleHandleW(
    lpModuleName: LPCWSTR
) -> HMODULE {
    unsafe { ffi::GetModuleHandleW(lpModuleName) }
}

pub fn CreatePopupMenu() -> HMENU {
    unsafe { ffi::CreatePopupMenu() }
}

pub fn AppendMenuA(
    hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT, lpNewItem: LPCSTR
) -> BOOL {
    unsafe { ffi::AppendMenuA(hMenu, uFlags, uIDNewItem, lpNewItem) }
}

pub fn AppendMenuW(
    hMenu: HMENU, uFlags: UINT, uIDNewItem: UINT, lpNewItem: LPCWSTR
) -> BOOL {
    unsafe { ffi::AppendMenuW(hMenu, uFlags, uIDNewItem, lpNewItem) }
}

pub fn GetCursorPos(
    lpPoint: LPPOINT
) -> BOOL {
    unsafe { ffi::GetCursorPos(lpPoint) }
}

pub fn SetForegroundWindow(
    hWnd: HWND
) -> BOOL {
    unsafe { ffi::SetForegroundWindow(hWnd) }
}

pub fn TrackPopupMenu(
    hMenu: HMENU, uFlags: UINT, x: c_int, y: c_int, nReserved: c_int, hWnd: HWND, prcRect: *mut RECT
) -> BOOL {
    unsafe { ffi::TrackPopupMenu(hMenu, uFlags, x, y, nReserved, hWnd, prcRect) }
}

pub fn DestroyWindow(
    hWnd: HWND
) -> BOOL {
    unsafe { ffi::DestroyWindow(hWnd) }
}