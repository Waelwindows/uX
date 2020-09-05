
pub trait Identity: Sized {
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_identity {
    ($($type: ty),+) => {
        $(
            impl Identity for $type {
                const ZERO: Self = 0;
                const ONE: Self = 1;
            }
        )+
    };
}

impl_identity!(i8, i16, i32, i64, i128);
impl_identity!(u8, u16, u32, u64, u128);
impl_identity!(usize, isize);
