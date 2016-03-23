use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::slice;

use winapi::minwindef::*;
use winapi::winnt::*;

pub type Win32Result<T> = Result<T, DWORD>;

pub trait WinapiOsStringExt {
	unsafe fn from_wide_ptr(ptr: LPCWSTR, len: usize) -> Self;
}

impl WinapiOsStringExt for OsString {
	unsafe fn from_wide_ptr(ptr: LPCWSTR, len: usize) -> Self {
		let slice: &[u16] = slice::from_raw_parts(ptr, len);

		OsStringExt::from_wide(slice)
	}
}
