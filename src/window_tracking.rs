use std::collections::HashMap;
use std::default::Default;
use std::mem;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::basetsd::*;

use utils::Win32Result;

pub struct TrackedWindow {
    uint_hwnd: UINT_PTR,
    title: Option<String>,
}

impl TrackedWindow {
	pub unsafe fn new(hwnd: HWND, title: String) -> Self {
		TrackedWindow {
			uint_hwnd: mem::transmute(hwnd),
			title: Some(title),
		}
	}

	pub unsafe fn hwnd(&self) -> HWND {
		mem::transmute(self.uint_hwnd)
	}

	pub fn title(&self) -> Option<&str> {
		match self.title {
			Some(ref s) => Some(&s),
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
	let foreground_window = ::user32::GetForegroundWindow();

	if foreground_window == 0 as HWND {
		return Err(::kernel32::GetLastError());
	}

	let tracked_window = TrackedWindow::new(foreground_window, "".to_string());

	Ok(tracked_window)
}
