#![no_std]

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, FromPrimitive, ToPrimitive)]
pub enum StatusCode {
    Success = 0,
    NotFound = 1,
    PermissionDenied = 2,
    ConnectionRefused = 3,
    ConnectionReset = 4,
    ConnectionAborted = 5,
    NotConnected = 6,
    AddrInUse = 7,
    AddrNotAvailable = 8,
    BrokenPipe = 9,
    AlreadyExists = 10,
    WouldBlock = 11,
    InvalidInput = 12,
    InvalidData = 13,
    TimedOut = 14,
    WriteZero = 15,
    Interrupted = 16,
    Other = 17,
    UnexpectedEof = 18,
    Range = 19,
    Segfault = 20,
    TooManyResources = 21,
    BadMemory = 22,
    FrameUnderflow = 23,
    InvalidConversion = 24,
    InvalidTable = 25,
    InvalidMemory = 26,
    OutOfBoundGlobal = 27,
    OutOfBoundLocal = 28,
    OutOfBoundMemory = 29,
    OutOfMemory = 30,
    StackUnderflow = 31,
    StackOverflow = 32,
    Unreachable = 33,
    InvalidIntegerDivision = 34,
    InvalidIndirectSignature = 35,
}

impl From<u32> for StatusCode {
    fn from(value: u32) -> Self {
        FromPrimitive::from_u32(value as u32).unwrap_or(StatusCode::Other)
    }
}

impl Into<::core::result::Result<(), StatusCode>> for StatusCode {
    fn into(self) -> ::core::result::Result<(), StatusCode> {
        match self {
            StatusCode::Success => Ok(()),
            _ => Err(self),
        }
    }
}
