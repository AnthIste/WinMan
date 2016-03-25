use std::marker::PhantomData;
use std::mem;

use winapi::basetsd::*;
use winapi::minwindef::*;
use winapi::windef::*;

pub type Win32Result<T> = Result<T, DWORD>;

pub struct SendHandle<H> {
	uint_handle: UINT_PTR,
	_h: PhantomData<H>,
}

unsafe impl Send for SendHandle<HWND> { }

impl SendHandle<HWND> {
	pub fn new(handle: HWND) -> Self {
		SendHandle {
			uint_handle: unsafe { mem::transmute(handle) },
			_h: PhantomData,
		}		
	}

	pub fn handle(&self) -> HWND {
		unsafe { mem::transmute(self.uint_handle) }
	}
}
