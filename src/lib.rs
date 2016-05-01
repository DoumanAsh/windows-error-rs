//! Windows Error
//!
//! This crate provide simple means to operate WinAPI errors.
//!


extern crate winapi;
extern crate kernel32;

use winapi::{DWORD};
use winapi::winbase::{
    FORMAT_MESSAGE_ARGUMENT_ARRAY,
    FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS
};
use kernel32::{
    FormatMessageW,
    GetLastError
};

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

    ///Returns description of underlying error code.
    pub fn errno_desc(&self) -> String {
        const BUF_SIZE: usize = 512;
        const FMT_FLAGS: DWORD = FORMAT_MESSAGE_IGNORE_INSERTS | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ARGUMENT_ARRAY;
        let mut format_buff: [u16; BUF_SIZE] = [0; BUF_SIZE];
        let num_chars: u32 = unsafe { FormatMessageW(FMT_FLAGS,
                                                     std::ptr::null(), self.0,
                                                     0, format_buff.as_mut_ptr(),
                                                     BUF_SIZE as u32, std::ptr::null_mut()) };

        let num_chars: usize = num_chars as usize;
        //Errors are formatted with windows new lines at the end.
        //If string does not end with /r/n then, most possibly, it is not a error
        //but some other system thing(who knows what...)
        if num_chars == 0 || format_buff[num_chars-1] != 10 {
            return "Unknown Error.".to_string();
        }
        String::from_utf16_lossy(&format_buff[0..num_chars-2])
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
