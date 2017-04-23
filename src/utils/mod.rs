use winapi;

pub mod api_wrappers;

pub type Win32Result<T> = Result<T, winapi::DWORD>;

// https://gist.github.com/sunnyone/e660fe7f73e2becd4b2c
pub fn from_wide_slice(buffer: &[u16]) -> String {
	use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

	let index = buffer.iter().position(|x| *x == 0).unwrap_or(buffer.len());
	let slice = unsafe { ::std::slice::from_raw_parts(buffer.as_ptr(), index) };

	OsString::from_wide(slice).to_string_lossy().into_owned()
}

// https://gist.github.com/sunnyone/e660fe7f73e2becd4b2c
pub fn to_wide_chars(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s)
		.encode_wide()
		.chain(::std::iter::once(0))
		.collect()
}