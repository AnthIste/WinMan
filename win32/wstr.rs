use win32::types::WCHAR;

pub trait ToCWStr {
    fn to_c_wstr(&self) -> Vec<WCHAR>;
}

impl<'a> ToCWStr for &'a str {
    fn to_c_wstr(&self) -> Vec<WCHAR> {
        self.utf16_units().collect()
    }
}

impl ToCWStr for String {
    fn to_c_wstr(&self) -> Vec<WCHAR> {
        self.as_slice().utf16_units().collect()
    }
}