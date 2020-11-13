use std::ffi::{OsStr, OsString};
#[cfg(target_os="windows")]
use std::os::windows::ffi::{OsStrExt, OsStringExt};

// Utilities to convert to and from windows text.

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
}

impl<T> ToWide for T
    where
        T: AsRef<OsStr>,
{
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}

pub trait FromWide
{
    fn from_wide(wide: &[u16]) -> Self;
}

impl FromWide for OsString {
    fn from_wide(wide: &[u16]) -> OsString {
        let len = wide.iter().take_while(|&&c| c != 0).count();
        OsStringExt::from_wide(&wide[..len])
    }
}
