use std::collections::{HashMap, VecDeque};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use kernel32;
use user32;
use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;

use utils::{SendHandle, Win32Result};

const MAX_TITLE_LEN: usize = 256;

#[derive(Clone)]
pub struct Window {
    hwnd: SendHandle<HWND>,
    title: Option<OsString>,
}

impl Window {
	pub fn new(hwnd: HWND, title: OsString) -> Self {
		Window {
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

pub struct WindowSet {
	windows: VecDeque<Window>
}

impl WindowSet {
	pub fn new() -> Self {
		WindowSet {
			windows: VecDeque::new()
		}
	}

	pub fn add(&mut self, window: Window) {
		let exists = self.windows
	                     .iter()
	                     .any(|w| w.hwnd == window.hwnd);

		if !exists {
			self.windows.push_front(window);
		}
	}
	
	pub fn remove(&mut self, window: &Window) -> Option<Window> {
		let index = self.windows
		                .iter()
		                .position(|w| w.hwnd == window.hwnd);

        match index {
        	Some(index) => {
        		self.windows.remove(index)
        	},
        	None => None
        }
	}

	pub fn cycle(&mut self) -> Option<Window> {
		if let Some(back) = self.windows.pop_back() {
			self.windows.push_front(back);
		}

		match self.windows.front() {
			Some(window) => Some(window.clone()),
			None => None
		}
	}
}

pub struct Config {
    windows: HashMap<UINT, WindowSet>
}

impl Config {
	pub fn new() -> Self {
		Config {
			windows: HashMap::new()
		}
	}

	pub fn track_window(&mut self, vk: UINT, window: Window) {
		let mut window_set = self.windows
		                         .entry(vk)
		                         .or_insert(WindowSet::new());

		window_set.add(window);		
	}

	pub fn get_windows(&mut self, vk: UINT) -> Option<&mut WindowSet> {
		self.windows.get_mut(&vk)
	}

	pub fn clear_windows(&mut self, vk: UINT) {
		self.windows.remove(&vk);
	}
}

pub fn get_foreground_window() -> Win32Result<Window> {
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
	let window = Window::new(foreground_window, title);

	Ok(window)
}

pub fn set_foreground_window(hwnd: HWND) -> Win32Result<()> {
	unsafe {
		if user32::SetForegroundWindow(hwnd) == 0 {
			return Err(kernel32::GetLastError());
		}
	}

	Ok(())
}
