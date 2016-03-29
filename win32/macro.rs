use win32::types::{UINT,WORD,DWORD,LPCSTR,LPCWSTR,ULONG_PTR};

pub fn MAKEINTRESOURCEA(i: UINT) -> LPCSTR {
    ((i as WORD) as ULONG_PTR) as LPCSTR
}

pub fn MAKEINTRESOURCEW(i: UINT) -> LPCWSTR {
    ((i as WORD) as ULONG_PTR) as LPCWSTR
}

pub fn HIWORD(u: DWORD) -> WORD {
    ((u & 0xFFFF0000) >> 16) as WORD
}

pub fn LOWORD(u: DWORD) -> WORD {
    (u & 0x0000FFFF) as WORD
}