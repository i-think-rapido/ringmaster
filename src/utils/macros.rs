
#[macro_export]
macro_rules! unsafe_clone_slices (
    () => {{ vec![] }};
    ($size:expr; $($e:expr),*) => {{
        unsafe fn clone<T>(list: Vec<&[T]>) -> Vec<std::mem::MaybeUninit<T>> {
            let mut sizes = vec![];
            let mut size = $size;

            for slice in &list {
                let mut len = slice.len();
                len = std::cmp::min(size, len);
                size -= len;
                sizes.push(len);
                if size == 0 {
                    break;
                }
            }

            let capacity = sizes.iter().sum();
            let mut out: Vec<std::mem::MaybeUninit<T>> = Vec::with_capacity(capacity);
            {
                out.set_len(capacity);

                let mut dst_ptr = out.as_mut_ptr() as *mut _ as *mut T;
                for (slice, len) in list.iter().zip(&sizes)
                {
                    let src_ptr = slice.as_ptr();
                    std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, *len);
                    dst_ptr = dst_ptr.add(*len);
                }
            }

            out
        }
        clone(vec![$($e),*])
    }};
    ($($e:expr),*) => {{
        unsafe_clone_slices!(usize::MAX; $($e),*)
    }};
);

