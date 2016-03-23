use std::collections::HashMap;
use std::default::Default;
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;

use kernel32;
use user32;
use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use winapi::basetsd::*;

use utils::Win32Result;

const MAX_TITLE_LEN: usize = 128;

pub struct TrackedWindow {
    uint_hwnd: UINT_PTR,
    title: Option<OsString>,
}

impl TrackedWindow {
	pub unsafe fn new(hwnd: HWND, title: OsString) -> Self {
		TrackedWindow {
			uint_hwnd: mem::transmute(hwnd),
			title: Some(title),
		}
	}

	pub unsafe fn hwnd(&self) -> HWND {
		mem::transmute(self.uint_hwnd)
	}

	pub fn title(&self) -> Option<&str> {
		// self.title.and_then(|t| t.to_str())
		match self.title {
			Some(ref t) => t.to_str(),
			None => None
		}
	}
}

impl Default for TrackedWindow {
	fn default() -> Self {
		TrackedWindow {
			uint_hwnd: 0,
			title: None,
		}
	}
}

#[derive(Default)]
pub struct Config {
    tracked_windows: HashMap<(UINT, UINT), TrackedWindow>,
}

impl Config {
	pub fn track_window(&mut self, modifiers: UINT, vk: UINT, tracked_window: TrackedWindow) {
		let key = (modifiers, vk);

		self.tracked_windows.insert(key, tracked_window);
	}
}

pub unsafe fn get_foreground_window() -> Win32Result<TrackedWindow> {
	let foreground_window = user32::GetForegroundWindow();

	if foreground_window == 0 as HWND {
		return Err(kernel32::GetLastError());
	}

	let mut title = [0 as WCHAR; MAX_TITLE_LEN];
	if user32::GetWindowTextW(foreground_window, title.as_mut_ptr(), MAX_TITLE_LEN as i32) == 0 {
		return Err(kernel32::GetLastError());
	}
	
	let title = OsStringExt::from_wide(&title);
	let tracked_window = TrackedWindow::new(foreground_window, title);

	Ok(tracked_window)
}
