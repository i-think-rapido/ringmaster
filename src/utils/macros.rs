
#[macro_export]
macro_rules! clone_slices(
    () => {{ vec![] }};
    ($size:expr; $($e:expr),*) => {{
        let list = vec![$($e),*];

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
        let mut out = Vec::with_capacity(capacity);
        unsafe {
            out.set_len(capacity);

            let mut dst_ptr = out.as_mut_ptr();
            for (slice, len) in list.iter().zip(&sizes)
            {
                let src_ptr = slice.as_ptr();
                std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, *len);
                dst_ptr = dst_ptr.add(*len);
            }
        }

        out
    }};
    ($($e:expr),*) => {{
        clone_slices!(usize::MAX; $($e),*)
    }};
);

