extern crate winapi;
extern crate kernel32;
extern crate user32;

use winapi::winuser::*;

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