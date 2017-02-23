#![cfg(windows)]
//! Windows Error
//!
//! This crate provide simple means to operate WinAPI errors.
//!

use std::os::raw::{
    c_ulong,
    c_void,
    c_ushort,
    c_char
};

type DWORD = c_ulong;

const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x00002000;
const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;

extern "system" {
    fn GetLastError() -> DWORD;
    fn FormatMessageW(dwFlags: DWORD,
                      lpSource: *const c_void,
                      dwMessageId: DWORD,
                      dwLanguageId: DWORD,
                      lpBuffer: *mut c_ushort,
                      nSize: DWORD,
                      Arguments: *mut c_char) -> DWORD;
}

fn format_message_error(buff: &[u16]) -> String {
    match unsafe {GetLastError()} {
        122 => String::from_utf16_lossy(buff), //Insufficient memory
        _ => "Unknown Error.".to_string()
    }
}

fn format_message_ok(buff: &[u16]) -> String {
    String::from_utf16_lossy(&buff[0..buff.len()-2])
}

///Returns description of error code.
///
///`Unknown Error.` is returned in case of bad error code.
pub fn format_error(errno: u32) -> String {
    const BUF_SIZE: usize = 512;
    const FMT_FLAGS: DWORD = FORMAT_MESSAGE_IGNORE_INSERTS | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ARGUMENT_ARRAY;
    let mut format_buff: [u16; BUF_SIZE] = [0; BUF_SIZE];

    let num_chars: u32 = unsafe { FormatMessageW(FMT_FLAGS,
                                                 std::ptr::null(), errno,
                                                 0, format_buff.as_mut_ptr(),
                                                 BUF_SIZE as u32, std::ptr::null_mut()) };

    if num_chars == 0 {
        format_message_error(&format_buff)
    } else {
        format_message_ok(&format_buff[0..num_chars as usize])
    }
}

use std::error::Error;
use std::fmt;

#[derive(Clone)]
///Represents Windows error code.
pub struct WindowsError(u32);

impl WindowsError {
    ///Constructs new error.
    pub fn new(errno: u32) -> WindowsError {
        WindowsError(errno)
    }

    ///Constructs new error from last happened one via ```GetLastError``` call.
    pub fn from_last_err() -> WindowsError {
        unsafe { WindowsError(GetLastError()) }
    }

    #[inline(always)]
    ///Returns underlying error code.
    pub fn errno(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    ///Returns description of underlying error code.
    pub fn errno_desc(&self) -> String {
        format_error(self.0)
    }
}

//Own debug as derive one is a bit lame
impl fmt::Debug for WindowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "WinAPI Error({})", self.errno())
    }
}

impl fmt::Display for WindowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "WinAPI Error({})", self.errno())
    }
}

impl Error for WindowsError {
    fn description(&self) -> &str {
        "WinAPI Error"
    }
}

impl PartialEq for WindowsError {
    fn eq(&self, right: &WindowsError) -> bool {
        self.0 == right.0
    }

    fn ne(&self, right: &WindowsError) -> bool {
        self.0 != right.0
    }
}

macro_rules! impl_traits
{
    ($($t:ty), +) => {
        $(
            impl From<$t> for WindowsError {
                fn from(num: $t) -> Self {
                    WindowsError(num as u32)
                }
            }

            impl Into<$t> for WindowsError {
                fn into(self) -> $t {
                    self.0 as $t
                }
            }

            impl PartialEq<$t> for WindowsError {
                fn eq(&self, right: &$t) -> bool {
                    self.0 == *right as u32
                }

                fn ne(&self, right: &$t) -> bool {
                    self.0 != *right as u32
                }
            }
        )+
    };
}

impl_traits!(u32, u16, u8, usize, i32, i16, i8, isize);
