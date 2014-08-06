use std::default::Default;

use win32::constants::*;
use win32::types::{HWND,UINT,NOTIFYICONDATA,WNDCLASSEXA,WNDPROC,HMENU,HINSTANCE,LPVOID,LPCSTR,HICON,
                   POINT,RECT};
use win32::window::{PostQuitMessage,GetModuleHandleA,Shell_NotifyIcon,RegisterClassExA,CreateWindowExA,
                    GetLastError,LoadImageW,GetSystemMetrics,SetForegroundWindow,TrackPopupMenu,
                    CreatePopupMenu,AppendMenuA,GetCursorPos};
use win32::macro::{MAKEINTRESOURCEW};

use app::window::{Win32Result,Win32Window};

// resource.h
static IDI_ICON1: UINT = 103;

pub struct DummyWindow {
    hInstance: HINSTANCE,
    hWnd: HWND,
    nid: Option<NOTIFYICONDATA>,
}

impl DummyWindow {
    pub fn create(hInstance: Option<HINSTANCE>, wndProc: WNDPROC) -> Win32Result<DummyWindow> {
        let hInstance = hInstance.unwrap_or(GetModuleHandleA(0 as LPCSTR));
        let className = "MyMagicClassName".to_c_str();

        let mut wc: WNDCLASSEXA = Default::default();
        wc.lpfnWndProc = wndProc;
        wc.lpszClassName = className.as_ptr();

        if RegisterClassExA(&wc) == 0 {
            return Err(GetLastError());
        }

        let hWnd = CreateWindowExA(
            0,
            className.as_ptr(),
            0 as LPCSTR,
            0,
            0,
            0,
            0,
            0,
            0 as HWND,
            0 as HMENU,
            hInstance,
            0 as LPVOID);

        if hWnd == 0 as HWND {
            return Err(GetLastError());
        }

        let mut dummy_window = DummyWindow {
            hInstance: hInstance,
            hWnd: hWnd,
            nid: None,
        };

        dummy_window.register_systray_icon();

        Ok(dummy_window)
    }

    pub fn register_systray_icon(&mut self) {
        match self.nid {
            None => {
                let mut nid: NOTIFYICONDATA = Default::default();

                nid.uID = 0x29A;
                nid.uCallbackMessage = 1234;
                nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
                nid.hWnd = self.hWnd;
                nid.hIcon = LoadImageW(
                    self.hInstance,
                    MAKEINTRESOURCEW(IDI_ICON1),
                    1, // IMAGE_ICON
                    GetSystemMetrics(49), // SM_CXSMICON
                    GetSystemMetrics(50), // SM_CYSMICON
                    0 // LR_DEFAULTCOLOR
                    ) as HICON;

                Shell_NotifyIcon(NIM_ADD, &mut nid);

                self.nid = Some(nid);
            }

            Some(_) => { }
        }
    }

    pub fn deregister_systray_icon(&mut self) {
        match self.nid {
            Some(mut nid) => {
                Shell_NotifyIcon(NIM_DELETE, &mut nid);
                self.nid = None;
            }

            None => { }
        }
    }

    pub fn show_popup_menu(&self) {
        let mut curPoint: POINT = Default::default();
        GetCursorPos(&mut curPoint);

        let hMenu = CreatePopupMenu();
        AppendMenuA(
            hMenu,
            0, // MF_STRING
            1, // TM_EXIT
            "E&xit".to_c_str().as_ptr()
            );

        SetForegroundWindow(self.hWnd);

        TrackPopupMenu(hMenu,
                       0x80 | 0x4 | 0x20, // TPM_NONOTIFY | TPM_CENTERALIGN | TPM_BOTTOMALIGN,
                       curPoint.x,
                       curPoint.y,
                       0,
                       self.hWnd,
                       0 as *mut RECT);
    }
}

impl Win32Window for DummyWindow {
    fn get_hwnd(&self) -> HWND {
        self.hWnd
    }

    fn get_hinstance(&self) -> Option<HINSTANCE> {
        Some(self.hInstance)
    }

    fn on_destroy(&mut self) -> bool {
        self.deregister_systray_icon();
        PostQuitMessage(0);
        true
    }
}