use crate::types::{AbiResult, StatusCode};
use wasp_abi::rand;

#[inline]
pub fn read(bytes: &mut [u8]) -> Result<u32, StatusCode> {
    let res: AbiResult = unsafe { rand::read(0, bytes.as_mut_ptr(), bytes.len() as u32) }.into();
    match res.into() {
        Ok(res) if bytes.len() > (res as usize) => read(&mut bytes[(res as usize)..]),
        other => other,
    }
}

#[inline]
pub fn u32() -> u32 {
    unsafe { rand::u32() }
}

#[inline]
pub fn u64() -> u64 {
    unsafe { rand::u64() }
}
