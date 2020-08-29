#![feature(const_generics)]
//! # uX - non-standard-width integers types
//!
//! When non-standard-width integers is required in an applications, the norm is to use a larger container and make sure the value is within range after manipulation. uX aims to take care of this once and for all by:
//!
//! - Providing `u1`-`u127` and `i1`-`i127` types that should behave as similar as possible to the built in rust types
//!     - The methods of the defined types are the same as for the built in types (far from all is implemented at this point but fill out an issue or create a PR if something essential for you is missing)
//!     - Overflow will panic in debug and wrap in release.
//! - All possible lossless conversions is possible by using `From`.
//! - When `TryFrom` is stabilized fallible conversions will also be supported.


#![cfg_attr(not(feature="std"), no_std)]

extern crate num_traits;

mod lib {
    pub mod core {
        #[cfg(feature="std")]
        pub use std::*;
        #[cfg(not(feature="std"))]
        pub use core::*;
    }
}

//mod conversion;

use lib::core::ops::{
    Shr,
    ShrAssign,
    Shl,
    ShlAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    BitAnd,
    BitAndAssign,
    Not
};

use lib::core::hash::{
    Hash,
    Hasher,
};

use lib::core::cmp::{
    Ordering,
    Ord,
    PartialOrd,
};

use lib::core::fmt::{
    Display,
    Formatter,
    UpperHex,
    LowerHex,
    Octal,
    Binary,
};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Unsigned<const BITS: usize, T=usize>(T);

impl<const N: usize, T> Unsigned<N, T> {
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }
}

impl<const N: usize> Unsigned<N, usize> {
    pub const MAX: Self = Self((1 << N)-1);
    pub const MIN: Self = Self(0);

    const fn mask(self) -> Self {
        Self(self.0 & ( (1usize << N).overflowing_sub(1).0))
    }

    /// Crates a new variable
    ///
    /// This function mainly exists as there is currently not a better way to construct these types.
    /// May be deprecated or removed if a better way to construct these types becomes available.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ux::*;
    ///
    /// assert_eq!(uint::<31>::new(64), uint::<31>::from(64u8));
    ///
    /// ```
    ///
    /// # Panic
    ///
    /// This function will panic if `value` is not representable by this type
    pub fn new(value: usize) -> Self {
        assert!(value <= Self::MAX.0 && value >= Self::MIN.0);
        unsafe { Self::new_unchecked(value) }
    }

    pub const fn try_new(value: usize) -> Option<Self> {
        if value <= Self::MAX.0 && value >= Self::MIN.0 {
            Some(unsafe { Self::new_unchecked(value) })
        } else {
            None
        }
    }
}

impl<const N: usize> Unsigned<N> {
    /// Wrapping (modular) subtraction. Computes `self - other`,
    /// wrapping around at the boundary of the type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ux::*;
    ///
    /// assert_eq!(i5::MIN.wrapping_sub(i5::new(1)), i5::MAX);
    ///
    /// assert_eq!(i5::new(-10).wrapping_sub(i5::new(5)), i5::new(-15));
    /// assert_eq!(i5::new(-15).wrapping_sub(i5::new(5)), i5::new(12));
    /// ```
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0)).mask()
    }

    /// Wrapping (modular) addition. Computes `self + other`,
    /// wrapping around at the boundary of the type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ux::*;
    ///
    /// assert_eq!(i5::MAX.wrapping_add(i5::new(1)), i5::MIN);
    ///
    /// assert_eq!(i5::new(10).wrapping_add(i5::new(5)), i5::new(15));
    /// assert_eq!(i5::new(15).wrapping_add(i5::new(5)), i5::new(-12));
    /// ```
    pub fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0)).mask()
    }
}

impl<T, const N: usize> Shl<T> for Unsigned<N>
where
    usize: Shl<T, Output=usize>
{
    type Output = Self;

    fn shl(self, rhs: T) -> Self {
        Self(self.mask().0.shl(rhs))
    }
}

impl<T, const N: usize> ShlAssign<T> for Unsigned<N>
where
    usize: ShlAssign<T>
{
    fn shl_assign(&mut self, rhs: T) {
        *self = self.mask();
        self.0.shl_assign(rhs);
    }
}

impl<T, const N: usize> Shr<T> for Unsigned<N>
where
    usize: Shr<T, Output=usize>
{
    type Output = Self;

    fn shr(self, rhs: T) -> Self {
        Self(self.mask().0.shr(rhs))
    }
}

impl<T, const N: usize> ShrAssign<T> for Unsigned<N>
where
    usize: ShrAssign<T>
{
    fn shr_assign(&mut self, rhs: T) {
        *self = self.mask();
        self.0.shr_assign(rhs);
    }
}

impl<const N: usize> BitOr<Unsigned<N>> for Unsigned<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.mask().0.bitor(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitOr<&'a Unsigned<N>> for Unsigned<N> {
    type Output = <Self as BitOr<Self>>::Output;

    fn bitor(self, rhs: &'a Unsigned<N>) -> Self::Output {
        Self(self.mask().0.bitor(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitOr<Unsigned<N>> for &'a Unsigned<N> {
    type Output = Unsigned<N>;

    fn bitor(self, rhs: Unsigned<N>) -> Self::Output {
        Unsigned(self.mask().0.bitor(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitOr<&'a Unsigned<N>> for &'a Unsigned<N> {
    type Output = Unsigned<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Unsigned(self.mask().0.bitor(rhs.mask().0))
    }
}

impl<const N: usize> BitOrAssign<Unsigned<N>> for Unsigned<N> {
    fn bitor_assign(&mut self, other: Unsigned<N>) {
        *self = self.mask();
        self.0.bitor_assign(other.mask().0)
    }
}

impl<const N: usize> BitXor<Unsigned<N>> for Unsigned<N> {
    type Output = Unsigned<N>;

    fn bitxor(self, rhs: Unsigned<N>) -> Self::Output {
        Self(self.mask().0.bitxor(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitXor<&'a Unsigned<N>> for Unsigned<N> {
    type Output = Unsigned<N>;

    fn bitxor(self, rhs: &'a Unsigned<N>) -> Self::Output {
        Self(self.mask().0.bitxor(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitXor<Unsigned<N>> for &'a Unsigned<N> {
    type Output = Unsigned<N>;

    fn bitxor(self, rhs: Unsigned<N>) -> Self::Output {
        Unsigned(self.mask().0.bitxor(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitXor<&'a Unsigned<N>> for &'a Unsigned<N> {
    type Output = Unsigned<N>;

    fn bitxor(self, rhs: &'a Unsigned<N>) -> Self::Output {
        Unsigned(self.mask().0.bitxor(rhs.mask().0))
    }
}

impl<const N: usize> BitXorAssign<Unsigned<N>> for Unsigned<N> {
    fn bitxor_assign(&mut self, other: Unsigned<N>) {
        *self = self.mask();
        self.0.bitxor_assign(other.mask().0)
    }
}

impl<const N: usize> Not for Unsigned<N> {
    type Output = Unsigned<N>;

    fn not(self) -> Unsigned<N> {
        Unsigned(self.mask().0.not())
    }
}

impl<'a, const N: usize> Not for &'a Unsigned<N> {
    type Output = <Unsigned<N> as Not>::Output;

    fn not(self) -> Unsigned<N> {
        Unsigned(self.mask().0.not())
    }
}

impl<const N: usize> BitAnd<Unsigned<N>> for Unsigned<N> {
    type Output = Unsigned<N>;

    fn bitand(self, rhs: Unsigned<N>) -> Self::Output {
        Unsigned(self.mask().0.bitand(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitAnd<&'a Unsigned<N>> for Unsigned<N> {
    type Output = <Unsigned<N> as BitOr<Unsigned<N>>>::Output;

    fn bitand(self, rhs: &'a Unsigned<N>) -> Self::Output {
        Unsigned(self.mask().0.bitand(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitAnd<Unsigned<N>> for &'a Unsigned<N> {
    type Output = <Unsigned<N> as BitOr<Unsigned<N>>>::Output;

    fn bitand(self, rhs: Unsigned<N>) -> Self::Output {
        Unsigned(self.mask().0.bitand(rhs.mask().0))
    }
}

impl<'a, const N: usize> BitAnd<&'a Unsigned<N>> for &'a Unsigned<N> {
    type Output = <Unsigned<N> as BitOr<Unsigned<N>>>::Output;

    fn bitand(self, rhs: &'a Unsigned<N>) -> Self::Output {
        Unsigned(self.mask().0.bitand(rhs.mask().0))
    }
}

impl<const N: usize> BitAndAssign<Unsigned<N>> for Unsigned<N> {
    fn bitand_assign(&mut self, other: Unsigned<N>) {
        *self = self.mask();
        self.0.bitand_assign(other.mask().0)
    }
}

impl<const N: usize> lib::core::ops::Add<Unsigned<N>> for Unsigned<N> {
    type Output = Unsigned<N>;
    #[allow(unused_comparisons)]
    fn add(self, other: Unsigned<N>) -> Unsigned<N> {
        if self.0 > 0 && other.0 > 0 {
            debug_assert!(Self::MAX.0 - other.0 >= self.0);
        } else if self.0 < 0 && other.0 < 0 {
            debug_assert!(Self::MIN.0 - other.0 <= self.0);
        }
        self.wrapping_add(other)
    }
}

impl<const N: usize> lib::core::ops::Sub<Unsigned<N>> for Unsigned<N> {
    type Output = Unsigned<N>;
    #[allow(unused_comparisons)]
    fn sub(self, other: Unsigned<N>) -> Unsigned<N> {
        if self > other {
            debug_assert!(Self::MAX.0 + other.0 >= self.0);
        } else if self < other {
            debug_assert!(Self::MIN.0 + other.0 <= self.0);
        }
        self.wrapping_sub(other)
    }
}

///The 1-bit unsigned integer type.
pub type u1 = Unsigned<1>;
///The 2-bit unsigned integer type.
pub type u2 = Unsigned<2>;
///The 3-bit unsigned integer type.
pub type u3 = Unsigned<3>;
///The 4-bit unsigned integer type.
pub type u4 = Unsigned<4>;
///The 5-bit unsigned integer type.
pub type u5 = Unsigned<5>;
///The 6-bit unsigned integer type.
pub type u6 = Unsigned<6>;
///The 7-bit unsigned integer type.
pub type u7 = Unsigned<7>;

///The 9-bit unsigned integer type.
pub type u9 = Unsigned<9>;
///The 10-bit unsigned integer type.
pub type u10 = Unsigned<10>;
///The 11-bit unsigned integer type.
pub type u11 = Unsigned<11>;
///The 12-bit unsigned integer type.
pub type u12 = Unsigned<12>;
///The 13-bit unsigned integer type.
pub type u13 = Unsigned<13>;
///The 14-bit unsigned integer type.
pub type u14 = Unsigned<14>;
///The 15-bit unsigned integer type.
pub type u15 = Unsigned<15>;

///The 17-bit unsigned integer type.
pub type u17 = Unsigned<17>;
///The 18-bit unsigned integer type.
pub type u18 = Unsigned<18>;
///The 19-bit unsigned integer type.
pub type u19 = Unsigned<19>;
///The 20-bit unsigned integer type.
pub type u20 = Unsigned<20>;
///The 21-bit unsigned integer type.
pub type u21 = Unsigned<21>;
///The 22-bit unsigned integer type.
pub type u22 = Unsigned<22>;
///The 23-bit unsigned integer type.
pub type u23 = Unsigned<23>;
///The 24-bit unsigned integer type.
pub type u24 = Unsigned<24>;
///The 25-bit unsigned integer type.
pub type u25 = Unsigned<25>;
///The 26-bit unsigned integer type.
pub type u26 = Unsigned<26>;
///The 27-bit unsigned integer type.
pub type u27 = Unsigned<27>;
///The 28-bit unsigned integer type.
pub type u28 = Unsigned<28>;
///The 29-bit unsigned integer type.
pub type u29 = Unsigned<29>;
///The 30-bit unsigned integer type.
pub type u30 = Unsigned<30>;
///The 31-bit unsigned integer type.
pub type u31 = Unsigned<31>;

///The 33-bit unsigned integer type.
pub type u33 = Unsigned<33>;
///The 34-bit unsigned integer type.
pub type u34 = Unsigned<34>;
///The 35-bit unsigned integer type.
pub type u35 = Unsigned<35>;
///The 36-bit unsigned integer type.
pub type u36 = Unsigned<36>;
///The 37-bit unsigned integer type.
pub type u37 = Unsigned<37>;
///The 38-bit unsigned integer type.
pub type u38 = Unsigned<38>;
///The 39-bit unsigned integer type.
pub type u39 = Unsigned<39>;
///The 40-bit unsigned integer type.
pub type u40 = Unsigned<40>;
///The 41-bit unsigned integer type.
pub type u41 = Unsigned<41>;
///The 42-bit unsigned integer type.
pub type u42 = Unsigned<42>;
///The 43-bit unsigned integer type.
pub type u43 = Unsigned<43>;
///The 44-bit unsigned integer type.
pub type u44 = Unsigned<44>;
///The 45-bit unsigned integer type.
pub type u45 = Unsigned<45>;
///The 46-bit unsigned integer type.
pub type u46 = Unsigned<46>;
///The 47-bit unsigned integer type.
pub type u47 = Unsigned<47>;
///The 48-bit unsigned integer type.
pub type u48 = Unsigned<48>;
///The 49-bit unsigned integer type.
pub type u49 = Unsigned<49>;
///The 50-bit unsigned integer type.
pub type u50 = Unsigned<50>;
///The 51-bit unsigned integer type.
pub type u51 = Unsigned<51>;
///The 52-bit unsigned integer type.
pub type u52 = Unsigned<52>;
///The 53-bit unsigned integer type.
pub type u53 = Unsigned<53>;
///The 54-bit unsigned integer type.
pub type u54 = Unsigned<54>;
///The 55-bit unsigned integer type.
pub type u55 = Unsigned<55>;
///The 56-bit unsigned integer type.
pub type u56 = Unsigned<56>;
///The 57-bit unsigned integer type.
pub type u57 = Unsigned<57>;
///The 58-bit unsigned integer type.
pub type u58 = Unsigned<58>;
///The 59-bit unsigned integer type.
pub type u59 = Unsigned<59>;
///The 60-bit unsigned integer type.
pub type u60 = Unsigned<60>;
///The 61-bit unsigned integer type.
pub type u61 = Unsigned<61>;
///The 62-bit unsigned integer type.
pub type u62 = Unsigned<62>;
///The 63-bit unsigned integer type.
pub type u63 = Unsigned<63>;

//define_unsigned!(#[doc="The 1-bit unsigned integer type."], u1, 1, u8);
//define_unsigned!(#[doc="The 2-bit unsigned integer type."], u2, 2, u8);
//define_unsigned!(#[doc="The 3-bit unsigned integer type."], u3, 3, u8);
//define_unsigned!(#[doc="The 4-bit unsigned integer type."], u4, 4, u8);
//define_unsigned!(#[doc="The 5-bit unsigned integer type."], u5, 5, u8);
//define_unsigned!(#[doc="The 6-bit unsigned integer type."], u6, 6, u8);
//define_unsigned!(#[doc="The 7-bit unsigned integer type."], u7, 7, u8);

//define_unsigned!(#[doc="The 9-bit unsigned integer type."], u9, 9, u16);
//define_unsigned!(#[doc="The 10-bit unsigned integer type."], u10, 10, u16);
//define_unsigned!(#[doc="The 11-bit unsigned integer type."], u11, 11, u16);
//define_unsigned!(#[doc="The 12-bit unsigned integer type."], u12, 12, u16);
//define_unsigned!(#[doc="The 13-bit unsigned integer type."], u13, 13, u16);
//define_unsigned!(#[doc="The 14-bit unsigned integer type."], u14, 14, u16);
//define_unsigned!(#[doc="The 15-bit unsigned integer type."], u15, 15, u16);

//define_unsigned!(#[doc="The 17-bit unsigned integer type."], u17, 17, u32);
//define_unsigned!(#[doc="The 18-bit unsigned integer type."], u18, 18, u32);
//define_unsigned!(#[doc="The 19-bit unsigned integer type."], u19, 19, u32);
//define_unsigned!(#[doc="The 20-bit unsigned integer type."], u20, 20, u32);
//define_unsigned!(#[doc="The 21-bit unsigned integer type."], u21, 21, u32);
//define_unsigned!(#[doc="The 22-bit unsigned integer type."], u22, 22, u32);
//define_unsigned!(#[doc="The 23-bit unsigned integer type."], u23, 23, u32);
//define_unsigned!(#[doc="The 24-bit unsigned integer type."], u24, 24, u32);

//define_unsigned!(#[doc="The 25-bit unsigned integer type."], u25, 25, u32);
//define_unsigned!(#[doc="The 26-bit unsigned integer type."], u26, 26, u32);
//define_unsigned!(#[doc="The 27-bit unsigned integer type."], u27, 27, u32);
//define_unsigned!(#[doc="The 28-bit unsigned integer type."], u28, 28, u32);
//define_unsigned!(#[doc="The 29-bit unsigned integer type."], u29, 29, u32);
//define_unsigned!(#[doc="The 30-bit unsigned integer type."], u30, 30, u32);
//define_unsigned!(#[doc="The 31-bit unsigned integer type."], u31, 31, u32);

//define_unsigned!(#[doc="The 33-bit unsigned integer type."], u33, 33, u64);
//define_unsigned!(#[doc="The 34-bit unsigned integer type."], u34, 34, u64);
//define_unsigned!(#[doc="The 35-bit unsigned integer type."], u35, 35, u64);
//define_unsigned!(#[doc="The 36-bit unsigned integer type."], u36, 36, u64);
//define_unsigned!(#[doc="The 37-bit unsigned integer type."], u37, 37, u64);
//define_unsigned!(#[doc="The 38-bit unsigned integer type."], u38, 38, u64);
//define_unsigned!(#[doc="The 39-bit unsigned integer type."], u39, 39, u64);
//define_unsigned!(#[doc="The 40-bit unsigned integer type."], u40, 40, u64);

//define_unsigned!(#[doc="The 41-bit unsigned integer type."], u41, 41, u64);
//define_unsigned!(#[doc="The 42-bit unsigned integer type."], u42, 42, u64);
//define_unsigned!(#[doc="The 43-bit unsigned integer type."], u43, 43, u64);
//define_unsigned!(#[doc="The 44-bit unsigned integer type."], u44, 44, u64);
//define_unsigned!(#[doc="The 45-bit unsigned integer type."], u45, 45, u64);
//define_unsigned!(#[doc="The 46-bit unsigned integer type."], u46, 46, u64);
//define_unsigned!(#[doc="The 47-bit unsigned integer type."], u47, 47, u64);
//define_unsigned!(#[doc="The 48-bit unsigned integer type."], u48, 48, u64);

//define_unsigned!(#[doc="The 49-bit unsigned integer type."], u49, 49, u64);
//define_unsigned!(#[doc="The 50-bit unsigned integer type."], u50, 50, u64);
//define_unsigned!(#[doc="The 51-bit unsigned integer type."], u51, 51, u64);
//define_unsigned!(#[doc="The 52-bit unsigned integer type."], u52, 52, u64);
//define_unsigned!(#[doc="The 53-bit unsigned integer type."], u53, 53, u64);
//define_unsigned!(#[doc="The 54-bit unsigned integer type."], u54, 54, u64);
//define_unsigned!(#[doc="The 55-bit unsigned integer type."], u55, 55, u64);
//define_unsigned!(#[doc="The 56-bit unsigned integer type."], u56, 56, u64);

//define_unsigned!(#[doc="The 57-bit unsigned integer type."], u57, 57, u64);
//define_unsigned!(#[doc="The 58-bit unsigned integer type."], u58, 58, u64);
//define_unsigned!(#[doc="The 59-bit unsigned integer type."], u59, 59, u64);
//define_unsigned!(#[doc="The 60-bit unsigned integer type."], u60, 60, u64);
//define_unsigned!(#[doc="The 61-bit unsigned integer type."], u61, 61, u64);
//define_unsigned!(#[doc="The 62-bit unsigned integer type."], u62, 62, u64);
//define_unsigned!(#[doc="The 63-bit unsigned integer type."], u63, 63, u64);

//define_unsigned!(#[doc="The 65-bit unsigned integer type."], u65, 65, u128);
//define_unsigned!(#[doc="The 66-bit unsigned integer type."], u66, 66, u128);
//define_unsigned!(#[doc="The 67-bit unsigned integer type."], u67, 67, u128);
//define_unsigned!(#[doc="The 68-bit unsigned integer type."], u68, 68, u128);
//define_unsigned!(#[doc="The 69-bit unsigned integer type."], u69, 69, u128);
//define_unsigned!(#[doc="The 70-bit unsigned integer type."], u70, 70, u128);
//define_unsigned!(#[doc="The 71-bit unsigned integer type."], u71, 71, u128);
//define_unsigned!(#[doc="The 72-bit unsigned integer type."], u72, 72, u128);

//define_unsigned!(#[doc="The 73-bit unsigned integer type."], u73, 73, u128);
//define_unsigned!(#[doc="The 74-bit unsigned integer type."], u74, 74, u128);
//define_unsigned!(#[doc="The 75-bit unsigned integer type."], u75, 75, u128);
//define_unsigned!(#[doc="The 76-bit unsigned integer type."], u76, 76, u128);
//define_unsigned!(#[doc="The 77-bit unsigned integer type."], u77, 77, u128);
//define_unsigned!(#[doc="The 78-bit unsigned integer type."], u78, 78, u128);
//define_unsigned!(#[doc="The 79-bit unsigned integer type."], u79, 79, u128);
//define_unsigned!(#[doc="The 80-bit unsigned integer type."], u80, 80, u128);

//define_unsigned!(#[doc="The 81-bit unsigned integer type."], u81, 81, u128);
//define_unsigned!(#[doc="The 82-bit unsigned integer type."], u82, 82, u128);
//define_unsigned!(#[doc="The 83-bit unsigned integer type."], u83, 83, u128);
//define_unsigned!(#[doc="The 84-bit unsigned integer type."], u84, 84, u128);
//define_unsigned!(#[doc="The 85-bit unsigned integer type."], u85, 85, u128);
//define_unsigned!(#[doc="The 86-bit unsigned integer type."], u86, 86, u128);
//define_unsigned!(#[doc="The 87-bit unsigned integer type."], u87, 87, u128);
//define_unsigned!(#[doc="The 88-bit unsigned integer type."], u88, 88, u128);

//define_unsigned!(#[doc="The 89-bit unsigned integer type."], u89, 89, u128);
//define_unsigned!(#[doc="The 90-bit unsigned integer type."], u90, 90, u128);
//define_unsigned!(#[doc="The 91-bit unsigned integer type."], u91, 91, u128);
//define_unsigned!(#[doc="The 92-bit unsigned integer type."], u92, 92, u128);
//define_unsigned!(#[doc="The 93-bit unsigned integer type."], u93, 93, u128);
//define_unsigned!(#[doc="The 94-bit unsigned integer type."], u94, 94, u128);
//define_unsigned!(#[doc="The 95-bit unsigned integer type."], u95, 95, u128);
//define_unsigned!(#[doc="The 96-bit unsigned integer type."], u96, 96, u128);

//define_unsigned!(#[doc="The 97-bit unsigned integer type."], u97, 97, u128);
//define_unsigned!(#[doc="The 98-bit unsigned integer type."], u98, 98, u128);
//define_unsigned!(#[doc="The 99-bit unsigned integer type."], u99, 99, u128);
//define_unsigned!(#[doc="The 100-bit unsigned integer type."], u100, 100, u128);
//define_unsigned!(#[doc="The 101-bit unsigned integer type."], u101, 101, u128);
//define_unsigned!(#[doc="The 102-bit unsigned integer type."], u102, 102, u128);
//define_unsigned!(#[doc="The 103-bit unsigned integer type."], u103, 103, u128);
//define_unsigned!(#[doc="The 104-bit unsigned integer type."], u104, 104, u128);

//define_unsigned!(#[doc="The 105-bit unsigned integer type."], u105, 105, u128);
//define_unsigned!(#[doc="The 106-bit unsigned integer type."], u106, 106, u128);
//define_unsigned!(#[doc="The 107-bit unsigned integer type."], u107, 107, u128);
//define_unsigned!(#[doc="The 108-bit unsigned integer type."], u108, 108, u128);
//define_unsigned!(#[doc="The 109-bit unsigned integer type."], u109, 109, u128);
//define_unsigned!(#[doc="The 110-bit unsigned integer type."], u110, 110, u128);
//define_unsigned!(#[doc="The 111-bit unsigned integer type."], u111, 111, u128);
//define_unsigned!(#[doc="The 112-bit unsigned integer type."], u112, 112, u128);

//define_unsigned!(#[doc="The 113-bit unsigned integer type."], u113, 113, u128);
//define_unsigned!(#[doc="The 114-bit unsigned integer type."], u114, 114, u128);
//define_unsigned!(#[doc="The 115-bit unsigned integer type."], u115, 115, u128);
//define_unsigned!(#[doc="The 116-bit unsigned integer type."], u116, 116, u128);
//define_unsigned!(#[doc="The 117-bit unsigned integer type."], u117, 117, u128);
//define_unsigned!(#[doc="The 118-bit unsigned integer type."], u118, 118, u128);
//define_unsigned!(#[doc="The 119-bit unsigned integer type."], u119, 119, u128);
//define_unsigned!(#[doc="The 120-bit unsigned integer type."], u120, 120, u128);

//define_unsigned!(#[doc="The 121-bit unsigned integer type."], u121, 121, u128);
//define_unsigned!(#[doc="The 122-bit unsigned integer type."], u122, 122, u128);
//define_unsigned!(#[doc="The 123-bit unsigned integer type."], u123, 123, u128);
//define_unsigned!(#[doc="The 124-bit unsigned integer type."], u124, 124, u128);
//define_unsigned!(#[doc="The 125-bit unsigned integer type."], u125, 125, u128);
//define_unsigned!(#[doc="The 126-bit unsigned integer type."], u126, 126, u128);
//define_unsigned!(#[doc="The 127-bit unsigned integer type."], u127, 127, u128);


//define_signed!(#[doc="The 1-bit signed integer type."], i1, 1, i8);
//define_signed!(#[doc="The 2-bit signed integer type."], i2, 2, i8);
//define_signed!(#[doc="The 3-bit signed integer type."], i3, 3, i8);
//define_signed!(#[doc="The 4-bit signed integer type."], i4, 4, i8);
//define_signed!(#[doc="The 5-bit signed integer type."], i5, 5, i8);
//define_signed!(#[doc="The 6-bit signed integer type."], i6, 6, i8);
//define_signed!(#[doc="The 7-bit signed integer type."], i7, 7, i8);

//define_signed!(#[doc="The 9-bit signed integer type."], i9, 9, i16);
//define_signed!(#[doc="The 10-bit signed integer type."], i10, 10, i16);
//define_signed!(#[doc="The 11-bit signed integer type."], i11, 11, i16);
//define_signed!(#[doc="The 12-bit signed integer type."], i12, 12, i16);
//define_signed!(#[doc="The 13-bit signed integer type."], i13, 13, i16);
//define_signed!(#[doc="The 14-bit signed integer type."], i14, 14, i16);
//define_signed!(#[doc="The 15-bit signed integer type."], i15, 15, i16);

//define_signed!(#[doc="The 17-bit signed integer type."], i17, 17, i32);
//define_signed!(#[doc="The 18-bit signed integer type."], i18, 18, i32);
//define_signed!(#[doc="The 19-bit signed integer type."], i19, 19, i32);
//define_signed!(#[doc="The 20-bit signed integer type."], i20, 20, i32);
//define_signed!(#[doc="The 21-bit signed integer type."], i21, 21, i32);
//define_signed!(#[doc="The 22-bit signed integer type."], i22, 22, i32);
//define_signed!(#[doc="The 23-bit signed integer type."], i23, 23, i32);
//define_signed!(#[doc="The 24-bit signed integer type."], i24, 24, i32);

//define_signed!(#[doc="The 25-bit signed integer type."], i25, 25, i32);
//define_signed!(#[doc="The 26-bit signed integer type."], i26, 26, i32);
//define_signed!(#[doc="The 27-bit signed integer type."], i27, 27, i32);
//define_signed!(#[doc="The 28-bit signed integer type."], i28, 28, i32);
//define_signed!(#[doc="The 29-bit signed integer type."], i29, 29, i32);
//define_signed!(#[doc="The 30-bit signed integer type."], i30, 30, i32);
//define_signed!(#[doc="The 31-bit signed integer type."], i31, 31, i32);

//define_signed!(#[doc="The 33-bit signed integer type."], i33, 33, i64);
//define_signed!(#[doc="The 34-bit signed integer type."], i34, 34, i64);
//define_signed!(#[doc="The 35-bit signed integer type."], i35, 35, i64);
//define_signed!(#[doc="The 36-bit signed integer type."], i36, 36, i64);
//define_signed!(#[doc="The 37-bit signed integer type."], i37, 37, i64);
//define_signed!(#[doc="The 38-bit signed integer type."], i38, 38, i64);
//define_signed!(#[doc="The 39-bit signed integer type."], i39, 39, i64);
//define_signed!(#[doc="The 40-bit signed integer type."], i40, 40, i64);

//define_signed!(#[doc="The 41-bit signed integer type."], i41, 41, i64);
//define_signed!(#[doc="The 42-bit signed integer type."], i42, 42, i64);
//define_signed!(#[doc="The 43-bit signed integer type."], i43, 43, i64);
//define_signed!(#[doc="The 44-bit signed integer type."], i44, 44, i64);
//define_signed!(#[doc="The 45-bit signed integer type."], i45, 45, i64);
//define_signed!(#[doc="The 46-bit signed integer type."], i46, 46, i64);
//define_signed!(#[doc="The 47-bit signed integer type."], i47, 47, i64);
//define_signed!(#[doc="The 48-bit signed integer type."], i48, 48, i64);

//define_signed!(#[doc="The 49-bit signed integer type."], i49, 49, i64);
//define_signed!(#[doc="The 50-bit signed integer type."], i50, 50, i64);
//define_signed!(#[doc="The 51-bit signed integer type."], i51, 51, i64);
//define_signed!(#[doc="The 52-bit signed integer type."], i52, 52, i64);
//define_signed!(#[doc="The 53-bit signed integer type."], i53, 53, i64);
//define_signed!(#[doc="The 54-bit signed integer type."], i54, 54, i64);
//define_signed!(#[doc="The 55-bit signed integer type."], i55, 55, i64);
//define_signed!(#[doc="The 56-bit signed integer type."], i56, 56, i64);

//define_signed!(#[doc="The 57-bit signed integer type."], i57, 57, i64);
//define_signed!(#[doc="The 58-bit signed integer type."], i58, 58, i64);
//define_signed!(#[doc="The 59-bit signed integer type."], i59, 59, i64);
//define_signed!(#[doc="The 60-bit signed integer type."], i60, 60, i64);
//define_signed!(#[doc="The 61-bit signed integer type."], i61, 61, i64);
//define_signed!(#[doc="The 62-bit signed integer type."], i62, 62, i64);
//define_signed!(#[doc="The 63-bit signed integer type."], i63, 63, i64);

//define_signed!(#[doc="The 65-bit signed integer type."], i65, 65, i128);
//define_signed!(#[doc="The 66-bit signed integer type."], i66, 66, i128);
//define_signed!(#[doc="The 67-bit signed integer type."], i67, 67, i128);
//define_signed!(#[doc="The 68-bit signed integer type."], i68, 68, i128);
//define_signed!(#[doc="The 69-bit signed integer type."], i69, 69, i128);
//define_signed!(#[doc="The 70-bit signed integer type."], i70, 70, i128);
//define_signed!(#[doc="The 71-bit signed integer type."], i71, 71, i128);
//define_signed!(#[doc="The 72-bit signed integer type."], i72, 72, i128);

//define_signed!(#[doc="The 73-bit signed integer type."], i73, 73, i128);
//define_signed!(#[doc="The 74-bit signed integer type."], i74, 74, i128);
//define_signed!(#[doc="The 75-bit signed integer type."], i75, 75, i128);
//define_signed!(#[doc="The 76-bit signed integer type."], i76, 76, i128);
//define_signed!(#[doc="The 77-bit signed integer type."], i77, 77, i128);
//define_signed!(#[doc="The 78-bit signed integer type."], i78, 78, i128);
//define_signed!(#[doc="The 79-bit signed integer type."], i79, 79, i128);
//define_signed!(#[doc="The 80-bit signed integer type."], i80, 80, i128);

//define_signed!(#[doc="The 81-bit signed integer type."], i81, 81, i128);
//define_signed!(#[doc="The 82-bit signed integer type."], i82, 82, i128);
//define_signed!(#[doc="The 83-bit signed integer type."], i83, 83, i128);
//define_signed!(#[doc="The 84-bit signed integer type."], i84, 84, i128);
//define_signed!(#[doc="The 85-bit signed integer type."], i85, 85, i128);
//define_signed!(#[doc="The 86-bit signed integer type."], i86, 86, i128);
//define_signed!(#[doc="The 87-bit signed integer type."], i87, 87, i128);
//define_signed!(#[doc="The 88-bit signed integer type."], i88, 88, i128);

//define_signed!(#[doc="The 89-bit signed integer type."], i89, 89, i128);
//define_signed!(#[doc="The 90-bit signed integer type."], i90, 90, i128);
//define_signed!(#[doc="The 91-bit signed integer type."], i91, 91, i128);
//define_signed!(#[doc="The 92-bit signed integer type."], i92, 92, i128);
//define_signed!(#[doc="The 93-bit signed integer type."], i93, 93, i128);
//define_signed!(#[doc="The 94-bit signed integer type."], i94, 94, i128);
//define_signed!(#[doc="The 95-bit signed integer type."], i95, 95, i128);
//define_signed!(#[doc="The 96-bit signed integer type."], i96, 96, i128);

//define_signed!(#[doc="The 97-bit signed integer type."], i97, 97, i128);
//define_signed!(#[doc="The 98-bit signed integer type."], i98, 98, i128);
//define_signed!(#[doc="The 99-bit signed integer type."], i99, 99, i128);
//define_signed!(#[doc="The 100-bit signed integer type."], i100, 100, i128);
//define_signed!(#[doc="The 101-bit signed integer type."], i101, 101, i128);
//define_signed!(#[doc="The 102-bit signed integer type."], i102, 102, i128);
//define_signed!(#[doc="The 103-bit signed integer type."], i103, 103, i128);
//define_signed!(#[doc="The 104-bit signed integer type."], i104, 104, i128);

//define_signed!(#[doc="The 105-bit signed integer type."], i105, 105, i128);
//define_signed!(#[doc="The 106-bit signed integer type."], i106, 106, i128);
//define_signed!(#[doc="The 107-bit signed integer type."], i107, 107, i128);
//define_signed!(#[doc="The 108-bit signed integer type."], i108, 108, i128);
//define_signed!(#[doc="The 109-bit signed integer type."], i109, 109, i128);
//define_signed!(#[doc="The 110-bit signed integer type."], i110, 110, i128);
//define_signed!(#[doc="The 111-bit signed integer type."], i111, 111, i128);
//define_signed!(#[doc="The 112-bit signed integer type."], i112, 112, i128);

//define_signed!(#[doc="The 113-bit signed integer type."], i113, 113, i128);
//define_signed!(#[doc="The 114-bit signed integer type."], i114, 114, i128);
//define_signed!(#[doc="The 115-bit signed integer type."], i115, 115, i128);
//define_signed!(#[doc="The 116-bit signed integer type."], i116, 116, i128);
//define_signed!(#[doc="The 117-bit signed integer type."], i117, 117, i128);
//define_signed!(#[doc="The 118-bit signed integer type."], i118, 118, i128);
//define_signed!(#[doc="The 119-bit signed integer type."], i119, 119, i128);
//define_signed!(#[doc="The 120-bit signed integer type."], i120, 120, i128);

//define_signed!(#[doc="The 121-bit signed integer type."], i121, 121, i128);
//define_signed!(#[doc="The 122-bit signed integer type."], i122, 122, i128);
//define_signed!(#[doc="The 123-bit signed integer type."], i123, 123, i128);
//define_signed!(#[doc="The 124-bit signed integer type."], i124, 124, i128);
//define_signed!(#[doc="The 125-bit signed integer type."], i125, 125, i128);
//define_signed!(#[doc="The 126-bit signed integer type."], i126, 126, i128);
//define_signed!(#[doc="The 127-bit signed integer type."], i127, 127, i128);


//#[cfg(test)]
//mod tests {
    //use super::*;

    //#[test]
    //fn test_masking() {
        //assert_eq!(u4(0b11000110).mask().0, 0b00000110);
        //assert_eq!(u4(0b00001000).mask().0, 0b00001000);
        //assert_eq!(u4(0b00001110).mask().0, 0b00001110);

        //assert_eq!(i4(0b11000110u8 as i8).mask().0, 0b00000110u8 as i8);
        //assert_eq!(i4(0b00001000u8 as i8).mask().0, 0b11111000u8 as i8);
        //assert_eq!(i4(0b00001110u8 as i8).mask().0, 0b11111110u8 as i8);

    //}

    //#[test]
    //fn min_max_values() {
        //assert_eq!(u1::MAX, u1(1));
        //assert_eq!(u2::MAX, u2(3));
        //assert_eq!(u3::MAX, u3(7));
        //assert_eq!(u7::MAX, u7(127));
        //assert_eq!(u9::MAX, u9(511));


        //assert_eq!(i1::MAX, i1(0));
        //assert_eq!(i2::MAX, i2(1));
        //assert_eq!(i3::MAX, i3(3));
        //assert_eq!(i7::MAX, i7(63));
        //assert_eq!(i9::MAX, i9(255));


        //assert_eq!(u1::MIN, u1(0));
        //assert_eq!(u2::MIN, u2(0));
        //assert_eq!(u3::MIN, u3(0));
        //assert_eq!(u7::MIN, u7(0));
        //assert_eq!(u9::MIN, u9(0));
        //assert_eq!(u127::MIN, u127(0));


        //assert_eq!(i1::MIN, i1(-1));
        //assert_eq!(i2::MIN, i2(-2));
        //assert_eq!(i3::MIN, i3(-4));
        //assert_eq!(i7::MIN, i7(-64));
        //assert_eq!(i9::MIN, i9(-256));


    //}

    //#[test]
    //fn test_wrapping_add() {
        //assert_eq!(u1::MAX.wrapping_add(u1(1)), u1(0));
        //assert_eq!(u1::MAX.wrapping_add(u1(0)), u1(1));

        //assert_eq!(u5::MAX.wrapping_add(u5(1)), u5(0));
        //assert_eq!(u5::MAX.wrapping_add(u5(4)), u5(3));

        //assert_eq!(u127::MAX.wrapping_add(u127(100)), u127(99));
        //assert_eq!(u127::MAX.wrapping_add(u127(1)), u127(0));

        //assert_eq!(i1::MAX.wrapping_add(i1(0)), i1(0));
        //assert_eq!(i1::MAX.wrapping_add(i1(-1)), i1(-1));

        //assert_eq!(i7::MAX.wrapping_add(i7(1)), i7::MIN);
        //assert_eq!(i7::MAX.wrapping_add(i7(4)), i7(-61));
    //}

    //#[test]
    //#[should_panic]
    //fn test_add_overflow_u5() {
        //let _s = u5::MAX + u5(1);
    //}

    //#[test]
    //#[should_panic]
    //fn test_add_overflow_u127() { let _s = u127::MAX + u127(1); }

    //#[test]
    //#[should_panic]
    //fn test_add_overflow_i96() { let _s = i96::MAX + i96(100); }

    //#[test]
    //#[should_panic]
    //fn test_add_underflow_i96() { let _s = i96::MIN + i96(-100); }

    //#[test]
    //#[should_panic]
    //fn test_add_underflow_i17() {
        //let _s = i17::MIN + i17(-1);
    //}

    //#[test]
    //fn test_add() {
        //assert_eq!(u5(1) + u5(2), u5(3));

        //assert_eq!(i7::MAX + i7::MIN, i7(-1));
        //assert_eq!(i7(4) + i7(-3), i7(1));
        //assert_eq!(i7(-4) + i7(3), i7(-1));
        //assert_eq!(i7(-3) + i7(-20), i7(-23));
    //}

    //#[test]
    //#[should_panic]
    //fn test_sub_overflow_i23() {
        //let _s = i23::MIN - i23::MAX;
    //}

    //#[test]
    //#[should_panic]
    //fn test_sub_underflow_u5() {
        //let _s = u5::MIN - u5(1);
    //}

    //#[test]
    //#[should_panic]
    //fn test_sub_underflow_i5() {
        //let _s = i5::MIN - i5(1);
    //}

    //#[test]
    //fn test_sub() {
        //assert_eq!(u5(1) - u5(1), u5(0));
        //assert_eq!(u5(3) - u5(2), u5(1));

        //assert_eq!(i1(-1) - i1(-1) , i1(0));
        //assert_eq!(i7::MIN - i7::MIN , i7(0));
        //assert_eq!(i7(4) - i7(-3), i7(7));
        //assert_eq!(i7(-4) - i7(3), i7(-7));
        //assert_eq!(i7(-3) - i7(-20), i7(17));
    //}

    //#[test]
    //fn test_shr() {
        //assert_eq!(u5(8) >> 1usize, u5(4));
        //assert_eq!(u5(8) >> 1u8, u5(4));
        //assert_eq!(u5(8) >> 1u16, u5(4));
        //assert_eq!(u5(8) >> 1u32, u5(4));
        //assert_eq!(u5(8) >> 1u64, u5(4));
        //assert_eq!(u5(8) >> 1isize, u5(4));
        //assert_eq!(u5(8) >> 1i8, u5(4));
        //assert_eq!(u5(8) >> 1i16, u5(4));
        //assert_eq!(u5(8) >> 1i32, u5(4));
        //assert_eq!(u5(8) >> 1i64, u5(4));

        //assert_eq!(u5::MAX >> 4, u5(1));

        //assert_eq!(i7(-1) >> 5, i7(-1));
    //}

    //#[test]
    //fn test_shl() {
        //assert_eq!(u5(16) << 1usize, u5(32));
        //assert_eq!(u5(16) << 1u8, u5(32));
        //assert_eq!(u5(16) << 1u16, u5(32));
        //assert_eq!(u5(16) << 1u32, u5(32));
        //assert_eq!(u5(16) << 1u64, u5(32));
        //assert_eq!(u5(16) << 1isize, u5(32));
        //assert_eq!(u5(16) << 1i8, u5(32));
        //assert_eq!(u5(16) << 1i16, u5(32));
        //assert_eq!(u5(16) << 1i32, u5(32));
        //assert_eq!(u5(16) << 1i64, u5(32));

        //assert_eq!(u5::MAX << 4, u5(16));

        //assert_eq!(i5(16) << 1, i5(0));
        //assert_eq!(i7(1) << 3, i7(8));
    //}

    //#[test]
    //fn test_shr_assign() {
        //let mut x = u10(512);
        //x >>= 1usize;
        //assert_eq!(x, u10(256));
        //x >>= 1isize;
        //assert_eq!(x, u10(128));
        //x >>= 1u8;
        //assert_eq!(x, u10(64));
        //x >>= 1i8;
        //assert_eq!(x, u10(32));
        //x >>= 2u64;
        //assert_eq!(x, u10(8));
        //x >>= 3i32;
        //assert_eq!(x, u10(1));
    //}

    //#[test]
    //fn test_shl_assign() {
        //let mut x = u9(1);
        //x <<= 3i32;
        //assert_eq!(x, u9(8));
        //x <<= 2u64;
        //assert_eq!(x, u9(32));
        //x <<= 1usize;
        //assert_eq!(x, u9(64));
        //x <<= 1isize;
        //assert_eq!(x, u9(128));
        //x <<= 1u8;
        //assert_eq!(x, u9(256));
    //}

    //#[test]
    //fn test_bitor() {
        //assert_eq!(u9(1) | u9(8), u9(9));
        //assert_eq!(&u9(1) | u9(8), u9(9));
        //assert_eq!(u9(1) | &u9(8), u9(9));
        //assert_eq!(&u9(1) | &u9(8), u9(9));
    //}

    //#[test]
    //fn test_bitor_assign() {
        //let mut x = u12(4);
        //x |= u12(1);
        //assert_eq!(x, u12(5));
        //x |= u12(128);
        //assert_eq!(x, u12(133));
        //x = u12(1);
        //x |= u12(127);
        //assert_eq!(x, u12(127));
    //}

    //#[test]
    //fn test_bitxor() {
        //assert_eq!(u7(0x7F) ^ u7(42), u7(85));
        //assert_eq!(&u7(0) ^ u7(42), u7(42));
        //assert_eq!(u7(0x10) ^ &u7(0x1), u7(0x11));
        //assert_eq!(&u7(11) ^ &u7(1), u7(10));
    //}

    //#[test]
    //fn test_bitxor_assign() {
        //let mut x = u12(4);
        //x ^= u12(1);
        //assert_eq!(x, u12(5));
        //x ^= u12(128);
        //assert_eq!(x, u12(133));
        //x ^= u12(1);
        //assert_eq!(x, u12(132));
        //x ^= u12(127);
        //assert_eq!(x, u12(251));
    //}

    //#[test]
    //fn test_bitand() {
        //assert_eq!(i9(-7) & i9(-9), i9::from(-7i8 & -9i8));
        //assert_eq!(&i9(-7) & i9(-9), i9::from(&-7i8 & -9i8));
        //assert_eq!(i9(-7) & &i9(-9), i9::from(-7i8 & &-9i8));
        //assert_eq!(&i9(-7) & &i9(-9), i9::from(&-7i8 & &-9i8));

        //assert_eq!(u9(8) & u9(9), u9(8));
        //assert_eq!(&u9(8) & u9(9), u9(8));
        //assert_eq!(u9(8) & &u9(9), u9(8));
        //assert_eq!(&u9(8) & &u9(9), u9(8));
    //}

    //#[test]
    //fn test_bitand_assign() {
        //let mut x = u12(255);
        //x &= u12(127);
        //assert_eq!(x, u12(127));
        //x &= u12(7);
        //assert_eq!(x, u12(7));
        //x &= u12(127);
        //assert_eq!(x, u12(7));
        //x &= u12(4);
        //assert_eq!(x, u12(4));
    //}

    //#[test]
    //fn test_not() {
        //assert_eq!(!u7(42), u7(85));
        //assert_eq!(!u7(0x7F), u7(0));
        //assert_eq!(!u7(0), u7(0x7F));
        //assert_eq!(!u7(56), u7(71));
    //}

//}
