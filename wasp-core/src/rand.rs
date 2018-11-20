use wasp_abi::rand;

#[inline]
pub fn u32() -> u32 {
    unsafe { rand::u32() }
}

#[inline]
pub fn u64() -> u64 {
    unsafe { rand::u64() }
}
