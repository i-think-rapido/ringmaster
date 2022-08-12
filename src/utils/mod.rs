
pub mod macros;

use std::ptr::copy_nonoverlapping;
use std::mem::MaybeUninit;
#[inline]
pub unsafe fn clone<T>(reference: &T) -> T {
    let mut out = MaybeUninit::<T>::uninit();
    let src = reference as *const _ as *const T;
    let dst = out.as_mut_ptr();
    copy_nonoverlapping(src, dst, 1);
    out.assume_init()
}

