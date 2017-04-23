use winapi::DWORD;

pub type Win32Result<T> = Result<T, DWORD>;