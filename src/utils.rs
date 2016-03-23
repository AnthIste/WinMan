use winapi::minwindef::*;

pub type Win32Result<T> = Result<T, DWORD>;
