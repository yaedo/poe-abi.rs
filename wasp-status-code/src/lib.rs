#![cfg_attr(not(feature = "std"), no_std)]
#![feature(try_trait)]

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
    InvalidFrame = 27,
    OutOfBoundGlobal = 28,
    OutOfBoundLocal = 29,
    OutOfBoundMemory = 30,
    OutOfMemory = 31,
    StackUnderflow = 32,
    StackOverflow = 33,
    Unreachable = 34,
    InvalidIntegerDivision = 35,
    InvalidIndirectSignature = 36,
    CPUExhaustion = 37,
}

macro_rules! conv {
    ($ty:ident, $call:ident) => {
        impl From<$ty> for StatusCode {
            fn from(value: $ty) -> Self {
                FromPrimitive::$call(value).unwrap_or(StatusCode::Other)
            }
        }

        impl Into<$ty> for StatusCode {
            fn into(self) -> $ty {
                self as $ty
            }
        }
    };
}

conv!(u8, from_u8);
conv!(u16, from_u16);
conv!(u32, from_u32);
conv!(u64, from_u64);
conv!(usize, from_usize);

impl Into<::core::result::Result<(), StatusCode>> for StatusCode {
    fn into(self) -> ::core::result::Result<(), StatusCode> {
        match self {
            StatusCode::Success => Ok(()),
            _ => Err(self),
        }
    }
}

impl core::ops::Try for StatusCode {
    type Error = Self;
    type Ok = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            StatusCode::Success => Ok(self),
            _ => Err(self),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        v
    }

    fn from_ok(v: Self::Error) -> Self {
        v
    }
}

macro_rules! io_error {
    ($($name:ident,)*) => {
        #[cfg(feature = "std")]
        impl From<std::io::ErrorKind> for StatusCode {
            fn from(kind: std::io::ErrorKind) -> Self {
                match kind {
                    $(std::io::ErrorKind::$name => StatusCode::$name,)*
                    _ => StatusCode::Other,
                }
            }
        }

        #[cfg(feature = "std")]
        impl From<std::io::Error> for StatusCode {
            fn from(error: std::io::Error) -> Self {
                error.kind().into()
            }
        }

        #[cfg(feature = "std")]
        impl Into<std::io::ErrorKind> for StatusCode {
            fn into(self) -> std::io::ErrorKind {
                match self {
                    $(StatusCode::$name => std::io::ErrorKind::$name,)*
                    _ => std::io::ErrorKind::Other,
                }
            }
        }

        #[cfg(feature = "std")]
        impl Into<std::io::Error> for StatusCode {
            fn into(self) -> std::io::Error {
                std::io::Error::new(self.into(), "")
            }
        }
    }
}

io_error!(
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    UnexpectedEof,
);
