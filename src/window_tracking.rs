use std::collections::HashMap;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use kernel32;
use user32;
use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;

use utils::{SendHandle, Win32Result};

const MAX_TITLE_LEN: usize = 256;

pub struct TrackedWindow {
    hwnd: SendHandle<HWND>,
    title: Option<OsString>,
}

impl TrackedWindow {
	pub fn new(hwnd: HWND, title: OsString) -> Self {
		TrackedWindow {
			hwnd: SendHandle::new(hwnd),
			title: Some(title),
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.hwnd.handle()
	}

	pub fn title(&self) -> Option<&str> {
		match self.title {
			Some(ref t) => t.to_str(),
			None => None
		}
	}
}

#[derive(Default)]
pub struct Config {
    tracked_windows: HashMap<UINT, TrackedWindow>,
}

impl Config {
	pub fn track_window(&mut self, vk: UINT, tracked_window: TrackedWindow) {
		self.tracked_windows.insert(vk, tracked_window);
	}

	pub fn get_tracked_window(&mut self, vk: UINT) -> Option<&TrackedWindow> {
		self.tracked_windows.get(&vk)
	}
}

pub fn get_foreground_window() -> Win32Result<TrackedWindow> {
	let (foreground_window, title) = unsafe {
		let foreground_window = user32::GetForegroundWindow();

		if foreground_window == 0 as HWND {
			return Err(kernel32::GetLastError());
		}

		let mut title = [0 as WCHAR; MAX_TITLE_LEN];
		if user32::GetWindowTextW(foreground_window, title.as_mut_ptr(), MAX_TITLE_LEN as i32) == 0 {
			return Err(kernel32::GetLastError());
		}

		(foreground_window, title)
	};
	
	let title = OsStringExt::from_wide(&title);
	let tracked_window = TrackedWindow::new(foreground_window, title);

	Ok(tracked_window)
}

pub fn set_foreground_window(hwnd: HWND) -> Win32Result<()> {
	unsafe {
		if user32::SetForegroundWindow(hwnd) == 0 {
			return Err(kernel32::GetLastError());
		}
	}

	Ok(())
}
