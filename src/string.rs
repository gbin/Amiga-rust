use crate::string;

mod mem {
    extern "rust-intrinsic" {
        #[rustc_const_stable(feature = "const_transmute", since = "1.46.0")]
        fn transmute<T, U>(_: T) -> U;
    }
}

struct FatPtr<T> {
    data: *const T,
    len: usize,
}

pub union Repr<T> {
    rust: *const [T],
    rust_mut: *mut [T],
    raw: FatPtr<T>,
}

impl<T> [T] {
    pub const fn len(&self) -> usize {
        unsafe { Repr { rust: self }.raw.len }
    }
}

impl str {
    pub const fn len(&self) -> usize {
        self.as_bytes().len()
    }

    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { mem::transmute(self) }
    }
}