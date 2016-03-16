extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use winapi::winuser::*;

type Win32Result<T> = Result<T, DWORD>;

pub fn main() {
	println!("Hello Windows!");

	let foreground_window = unsafe { user32::GetForegroundWindow() };

	println!("{:?}", foreground_window);

	unsafe {
		user32::ShowWindow(foreground_window, SW_HIDE);
		kernel32::Sleep(1000);
		user32::ShowWindow(foreground_window, SW_SHOW);
	}
}

fn _create_window() -> Win32Result<()> {
	// TODO: convert this to *const u16 ?? https://www.reddit.com/r/rust/comments/37k1ij/converting_str_to_utf16/
	let class_name: Vec<u16> = OsStr::new("MyMagicClassName").encode_wide().collect();

    let _window_class = WNDCLASSEXW {
    	cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
    	style: 0,
    	lpfnWndProc: Some(_window_proc),
    	cbClsExtra: 0,
    	cbWndExtra: 0,
    	hInstance: 0 as HINSTANCE,
    	hIcon: 0 as HICON,
    	hCursor: 0 as HCURSOR,
    	hbrBackground: 0 as HBRUSH,
    	lpszMenuName: 0 as LPCWSTR,
    	lpszClassName: class_name.as_ptr(),
    	hIconSm: 0 as HICON,
    };
 //    wc.lpfnWndProc = wndProc;
 //    wc.lpszClassName = className.as_ptr();

 //    if RegisterClassExA(&wc) == 0 {
 //        return Err(GetLastError());
 //    }

 //    let hWnd = CreateWindowExA(
 //        0,
 //        className.as_ptr(),
 //        0 as LPCSTR,
 //        0,
 //        0,
 //        0,
 //        0,
 //        0,
 //        0 as HWND,
 //        0 as HMENU,
 //        hInstance,
 //        0 as LPVOID);

 //    if hWnd == 0 as HWND {
 //        return Err(GetLastError());
 //    }
    Ok(())
}

unsafe extern "system" fn _window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let lresult = match msg {
        WM_CREATE               => Some(0),
        WM_DESTROY              => Some(0),
        WM_COMMAND              => Some(0),
        user if user >= WM_USER => Some(0),
        _                       => None
    };

    lresult.unwrap_or(user32::DefWindowProcW(hwnd, msg, wparam, lparam))
}
