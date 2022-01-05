#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn align_of_u8() -> usize {
    std::mem::align_of::<u8>()
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn align_of_u16() -> usize {
    std::mem::align_of::<u16>()
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn align_of_u32() -> usize {
    std::mem::align_of::<u32>()
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn align_of_u64() -> usize {
    std::mem::align_of::<u64>()
}

