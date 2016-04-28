use std::marker::PhantomData;
use std::mem;

use winapi::basetsd::*;
use winapi::minwindef::*;
use winapi::windef::*;

pub type Win32Result<T> = Result<T, DWORD>;

#[derive(Clone, Copy)]
pub struct SendHandle<H> {
	uint_handle: UINT_PTR,
	_h: PhantomData<H>,
}

unsafe impl<H> Send for SendHandle<H> { }

impl<H> Eq for SendHandle<H> { }

impl<H> PartialEq for SendHandle<H> {
	fn eq(&self, other: &Self) -> bool {
		self.uint_handle == other.uint_handle
	}
}

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
