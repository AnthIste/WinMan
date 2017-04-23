use kernel32;
use user32;
use winapi::minwindef::*;
use winapi::windef::*;

use utils;
use utils::Win32Result;

// https://github.com/retep998/wio-rs/blob/master/src/apc.rs
pub fn enum_windows<T>(func: T) -> Win32Result<()>
    where T: FnMut(HWND) -> BOOL {

    unsafe extern "system" fn helper<T: FnMut(HWND) -> BOOL>(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ppfn = lparam as *mut T;
        let mut func = &mut *ppfn;

        func(hwnd)
    }

    let result = unsafe {
        let ppfn = (&func) as *const T;
        user32::EnumWindows(Some(helper::<T>), ppfn as LPARAM)
    };

    match result {
        FALSE => match unsafe { kernel32::GetLastError() } {
            0 => Ok(()),
            err => Err(err)
        },
        _ => Ok(())
    }
}

pub fn get_window_text(hwnd: HWND) -> Win32Result<String> {
    use ::std::vec::Vec;

    // Get length of text to allocate buffer
    // The buffer must add space for a trailing null
    let buffer_size = unsafe {
        match user32::GetWindowTextLengthW(hwnd) {
            0 => {
                return Err(kernel32::GetLastError());
            },
            len => (len + 1) as usize
        }
    };

    // Allocate buffer and get text
    let mut buffer: Vec<u16> = Vec::with_capacity(buffer_size);
    unsafe {
        buffer.set_len(buffer_size);
        user32::GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32)
    };

    Ok(utils::from_wide_slice(&buffer))
}