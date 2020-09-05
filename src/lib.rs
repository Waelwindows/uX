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

#![allow(non_camel_case_types)]

#![cfg_attr(not(feature="std"), no_std)]

pub mod traits;
use crate::traits::*;
mod macros;

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
};

use lib::core::cmp::{
    Ord,
    PartialOrd,
};

#[derive(Debug, Default, Hash, Clone, Copy)]
pub struct Unsigned<const BITS: usize, T=u128>(T);

impl<const N: usize, T> Unsigned<N, T> {
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, Hash, Clone, Copy)]
pub struct Signed<const BITS: usize, T=u128>(T);

impl<const N: usize, T> Signed<N, T> {
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }
}

///The 1-bit unsigned integer type.
pub type u1 = Unsigned<1, u8>;
///The 2-bit unsigned integer type.
pub type u2 = Unsigned<2, u8>;
///The 3-bit unsigned integer type.
pub type u3 = Unsigned<3, u8>;
///The 4-bit unsigned integer type.
pub type u4 = Unsigned<4, u8>;
///The 5-bit unsigned integer type.
pub type u5 = Unsigned<5, u8>;
///The 6-bit unsigned integer type.
pub type u6 = Unsigned<6, u8>;
///The 7-bit unsigned integer type.
pub type u7 = Unsigned<7, u8>;

///The 9-bit unsigned integer type.
pub type u9 = Unsigned<9, u16>;
///The 10-bit unsigned integer type.
pub type u10 = Unsigned<10, u16>;
///The 11-bit unsigned integer type.
pub type u11 = Unsigned<11, u16>;
///The 12-bit unsigned integer type.
pub type u12 = Unsigned<12, u16>;
///The 13-bit unsigned integer type.
pub type u13 = Unsigned<13, u16>;
///The 14-bit unsigned integer type.
pub type u14 = Unsigned<14, u16>;
///The 15-bit unsigned integer type.
pub type u15 = Unsigned<15, u16>;

///The 17-bit unsigned integer type.
pub type u17 = Unsigned<17, u32>;
///The 18-bit unsigned integer type.
pub type u18 = Unsigned<18, u32>;
///The 19-bit unsigned integer type.
pub type u19 = Unsigned<19, u32>;
///The 20-bit unsigned integer type.
pub type u20 = Unsigned<20, u32>;
///The 21-bit unsigned integer type.
pub type u21 = Unsigned<21, u32>;
///The 22-bit unsigned integer type.
pub type u22 = Unsigned<22, u32>;
///The 23-bit unsigned integer type.
pub type u23 = Unsigned<23, u32>;
///The 24-bit unsigned integer type.
pub type u24 = Unsigned<24, u32>;
///The 25-bit unsigned integer type.
pub type u25 = Unsigned<25, u32>;
///The 26-bit unsigned integer type.
pub type u26 = Unsigned<26, u32>;
///The 27-bit unsigned integer type.
pub type u27 = Unsigned<27, u32>;
///The 28-bit unsigned integer type.
pub type u28 = Unsigned<28, u32>;
///The 29-bit unsigned integer type.
pub type u29 = Unsigned<29, u32>;
///The 30-bit unsigned integer type.
pub type u30 = Unsigned<30, u32>;
///The 31-bit unsigned integer type.
pub type u31 = Unsigned<31, u32>;

///The 33-bit unsigned integer type.
pub type u33 = Unsigned<33, u64>;
///The 34-bit unsigned integer type.
pub type u34 = Unsigned<34, u64>;
///The 35-bit unsigned integer type.
pub type u35 = Unsigned<35, u64>;
///The 36-bit unsigned integer type.
pub type u36 = Unsigned<36, u64>;
///The 37-bit unsigned integer type.
pub type u37 = Unsigned<37, u64>;
///The 38-bit unsigned integer type.
pub type u38 = Unsigned<38, u64>;
///The 39-bit unsigned integer type.
pub type u39 = Unsigned<39, u64>;
///The 40-bit unsigned integer type.
pub type u40 = Unsigned<40, u64>;
///The 41-bit unsigned integer type.
pub type u41 = Unsigned<41, u64>;
///The 42-bit unsigned integer type.
pub type u42 = Unsigned<42, u64>;
///The 43-bit unsigned integer type.
pub type u43 = Unsigned<43, u64>;
///The 44-bit unsigned integer type.
pub type u44 = Unsigned<44, u64>;
///The 45-bit unsigned integer type.
pub type u45 = Unsigned<45, u64>;
///The 46-bit unsigned integer type.
pub type u46 = Unsigned<46, u64>;
///The 47-bit unsigned integer type.
pub type u47 = Unsigned<47, u64>;
///The 48-bit unsigned integer type.
pub type u48 = Unsigned<48, u64>;
///The 49-bit unsigned integer type.
pub type u49 = Unsigned<49, u64>;
///The 50-bit unsigned integer type.
pub type u50 = Unsigned<50, u64>;
///The 51-bit unsigned integer type.
pub type u51 = Unsigned<51, u64>;
///The 52-bit unsigned integer type.
pub type u52 = Unsigned<52, u64>;
///The 53-bit unsigned integer type.
pub type u53 = Unsigned<53, u64>;
///The 54-bit unsigned integer type.
pub type u54 = Unsigned<54, u64>;
///The 55-bit unsigned integer type.
pub type u55 = Unsigned<55, u64>;
///The 56-bit unsigned integer type.
pub type u56 = Unsigned<56, u64>;
///The 57-bit unsigned integer type.
pub type u57 = Unsigned<57, u64>;
///The 58-bit unsigned integer type.
pub type u58 = Unsigned<58, u64>;
///The 59-bit unsigned integer type.
pub type u59 = Unsigned<59, u64>;
///The 60-bit unsigned integer type.
pub type u60 = Unsigned<60, u64>;
///The 61-bit unsigned integer type.
pub type u61 = Unsigned<61, u64>;
///The 62-bit unsigned integer type.
pub type u62 = Unsigned<62, u64>;
///The 63-bit unsigned integer type.
pub type u63 = Unsigned<63, u64>;

///The 65-bit unsigned integer type.
pub type u65 = Unsigned<65, u128>;
///The 66-bit unsigned integer type.
pub type u66 = Unsigned<66, u128>;
///The 67-bit unsigned integer type.
pub type u67 = Unsigned<67, u128>;
///The 68-bit unsigned integer type.
pub type u68 = Unsigned<68, u128>;
///The 69-bit unsigned integer type.
pub type u69 = Unsigned<69, u128>;
///The 70-bit unsigned integer type.
pub type u70 = Unsigned<70, u128>;
///The 71-bit unsigned integer type.
pub type u71 = Unsigned<71, u128>;
///The 72-bit unsigned integer type.
pub type u72 = Unsigned<72, u128>;
///The 73-bit unsigned integer type.
pub type u73 = Unsigned<73, u128>;
///The 74-bit unsigned integer type.
pub type u74 = Unsigned<74, u128>;
///The 75-bit unsigned integer type.
pub type u75 = Unsigned<75, u128>;
///The 76-bit unsigned integer type.
pub type u76 = Unsigned<76, u128>;
///The 77-bit unsigned integer type.
pub type u77 = Unsigned<77, u128>;
///The 78-bit unsigned integer type.
pub type u78 = Unsigned<78, u128>;
///The 79-bit unsigned integer type.
pub type u79 = Unsigned<79, u128>;
///The 80-bit unsigned integer type.
pub type u80 = Unsigned<80, u128>;
///The 81-bit unsigned integer type.
pub type u81 = Unsigned<81, u128>;
///The 82-bit unsigned integer type.
pub type u82 = Unsigned<82, u128>;
///The 83-bit unsigned integer type.
pub type u83 = Unsigned<83, u128>;
///The 84-bit unsigned integer type.
pub type u84 = Unsigned<84, u128>;
///The 85-bit unsigned integer type.
pub type u85 = Unsigned<85, u128>;
///The 86-bit unsigned integer type.
pub type u86 = Unsigned<86, u128>;
///The 87-bit unsigned integer type.
pub type u87 = Unsigned<87, u128>;
///The 88-bit unsigned integer type.
pub type u88 = Unsigned<88, u128>;
///The 89-bit unsigned integer type.
pub type u89 = Unsigned<89, u128>;
///The 90-bit unsigned integer type.
pub type u90 = Unsigned<90, u128>;
///The 91-bit unsigned integer type.
pub type u91 = Unsigned<91, u128>;
///The 92-bit unsigned integer type.
pub type u92 = Unsigned<92, u128>;
///The 93-bit unsigned integer type.
pub type u93 = Unsigned<93, u128>;
///The 94-bit unsigned integer type.
pub type u94 = Unsigned<94, u128>;
///The 95-bit unsigned integer type.
pub type u95 = Unsigned<95, u128>;
///The 96-bit unsigned integer type.
pub type u96 = Unsigned<96, u128>;
///The 97-bit unsigned integer type.
pub type u97 = Unsigned<97, u128>;
///The 98-bit unsigned integer type.
pub type u98 = Unsigned<98, u128>;
///The 99-bit unsigned integer type.
pub type u99 = Unsigned<99, u128>;
///The 100-bit unsigned integer type.
pub type u100 = Unsigned<100, u128>;
///The 101-bit unsigned integer type.
pub type u101 = Unsigned<101, u128>;
///The 102-bit unsigned integer type.
pub type u102 = Unsigned<102, u128>;
///The 103-bit unsigned integer type.
pub type u103 = Unsigned<103, u128>;
///The 104-bit unsigned integer type.
pub type u104 = Unsigned<104, u128>;
///The 105-bit unsigned integer type.
pub type u105 = Unsigned<105, u128>;
///The 106-bit unsigned integer type.
pub type u106 = Unsigned<106, u128>;
///The 107-bit unsigned integer type.
pub type u107 = Unsigned<107, u128>;
///The 108-bit unsigned integer type.
pub type u108 = Unsigned<108, u128>;
///The 109-bit unsigned integer type.
pub type u109 = Unsigned<109, u128>;
///The 110-bit unsigned integer type.
pub type u110 = Unsigned<110, u128>;
///The 111-bit unsigned integer type.
pub type u111 = Unsigned<111, u128>;
///The 112-bit unsigned integer type.
pub type u112 = Unsigned<112, u128>;
///The 113-bit unsigned integer type.
pub type u113 = Unsigned<113, u128>;
///The 114-bit unsigned integer type.
pub type u114 = Unsigned<114, u128>;
///The 115-bit unsigned integer type.
pub type u115 = Unsigned<115, u128>;
///The 116-bit unsigned integer type.
pub type u116 = Unsigned<116, u128>;
///The 117-bit unsigned integer type.
pub type u117 = Unsigned<117, u128>;
///The 118-bit unsigned integer type.
pub type u118 = Unsigned<118, u128>;
///The 119-bit unsigned integer type.
pub type u119 = Unsigned<119, u128>;
///The 120-bit unsigned integer type.
pub type u120 = Unsigned<120, u128>;
///The 121-bit unsigned integer type.
pub type u121 = Unsigned<121, u128>;
///The 122-bit unsigned integer type.
pub type u122 = Unsigned<122, u128>;
///The 123-bit unsigned integer type.
pub type u123 = Unsigned<123, u128>;
///The 124-bit unsigned integer type.
pub type u124 = Unsigned<124, u128>;
///The 125-bit unsigned integer type.
pub type u125 = Unsigned<125, u128>;
///The 126-bit unsigned integer type.
pub type u126 = Unsigned<126, u128>;
///The 127-bit unsigned integer type.
pub type u127 = Unsigned<127, u128>;


///The 1-bit signed integer type.
pub type i1 = Signed<1, i8>;
///The 2-bit signed integer type.
pub type i2 = Signed<2, i8>;
///The 3-bit signed integer type.
pub type i3 = Signed<3, i8>;
///The 4-bit signed integer type.
pub type i4 = Signed<4, i8>;
///The 5-bit signed integer type.
pub type i5 = Signed<5, i8>;
///The 6-bit signed integer type.
pub type i6 = Signed<6, i8>;
///The 7-bit signed integer type.
pub type i7 = Signed<7, i8>;

///The 9-bit signed integer type.
pub type i9 = Signed<9, i16>;
///The 10-bit signed integer type.
pub type i10 = Signed<10, i16>;
///The 11-bit signed integer type.
pub type i11 = Signed<11, i16>;
///The 12-bit signed integer type.
pub type i12 = Signed<12, i16>;
///The 13-bit signed integer type.
pub type i13 = Signed<13, i16>;
///The 14-bit signed integer type.
pub type i14 = Signed<14, i16>;
///The 15-bit signed integer type.
pub type i15 = Signed<15, i16>;

///The 17-bit signed integer type.
pub type i17 = Signed<17, i32>;
///The 18-bit signed integer type.
pub type i18 = Signed<18, i32>;
///The 19-bit signed integer type.
pub type i19 = Signed<19, i32>;
///The 20-bit signed integer type.
pub type i20 = Signed<20, i32>;
///The 21-bit signed integer type.
pub type i21 = Signed<21, i32>;
///The 22-bit signed integer type.
pub type i22 = Signed<22, i32>;
///The 23-bit signed integer type.
pub type i23 = Signed<23, i32>;
///The 24-bit signed integer type.
pub type i24 = Signed<24, i32>;
///The 25-bit signed integer type.
pub type i25 = Signed<25, i32>;
///The 26-bit signed integer type.
pub type i26 = Signed<26, i32>;
///The 27-bit signed integer type.
pub type i27 = Signed<27, i32>;
///The 28-bit signed integer type.
pub type i28 = Signed<28, i32>;
///The 29-bit signed integer type.
pub type i29 = Signed<29, i32>;
///The 30-bit signed integer type.
pub type i30 = Signed<30, i32>;
///The 31-bit signed integer type.
pub type i31 = Signed<31, i32>;

///The 33-bit signed integer type.
pub type i33 = Signed<33, i64>;
///The 34-bit signed integer type.
pub type i34 = Signed<34, i64>;
///The 35-bit signed integer type.
pub type i35 = Signed<35, i64>;
///The 36-bit signed integer type.
pub type i36 = Signed<36, i64>;
///The 37-bit signed integer type.
pub type i37 = Signed<37, i64>;
///The 38-bit signed integer type.
pub type i38 = Signed<38, i64>;
///The 39-bit signed integer type.
pub type i39 = Signed<39, i64>;
///The 40-bit signed integer type.
pub type i40 = Signed<40, i64>;
///The 41-bit signed integer type.
pub type i41 = Signed<41, i64>;
///The 42-bit signed integer type.
pub type i42 = Signed<42, i64>;
///The 43-bit signed integer type.
pub type i43 = Signed<43, i64>;
///The 44-bit signed integer type.
pub type i44 = Signed<44, i64>;
///The 45-bit signed integer type.
pub type i45 = Signed<45, i64>;
///The 46-bit signed integer type.
pub type i46 = Signed<46, i64>;
///The 47-bit signed integer type.
pub type i47 = Signed<47, i64>;
///The 48-bit signed integer type.
pub type i48 = Signed<48, i64>;
///The 49-bit signed integer type.
pub type i49 = Signed<49, i64>;
///The 50-bit signed integer type.
pub type i50 = Signed<50, i64>;
///The 51-bit signed integer type.
pub type i51 = Signed<51, i64>;
///The 52-bit signed integer type.
pub type i52 = Signed<52, i64>;
///The 53-bit signed integer type.
pub type i53 = Signed<53, i64>;
///The 54-bit signed integer type.
pub type i54 = Signed<54, i64>;
///The 55-bit signed integer type.
pub type i55 = Signed<55, i64>;
///The 56-bit signed integer type.
pub type i56 = Signed<56, i64>;
///The 57-bit signed integer type.
pub type i57 = Signed<57, i64>;
///The 58-bit signed integer type.
pub type i58 = Signed<58, i64>;
///The 59-bit signed integer type.
pub type i59 = Signed<59, i64>;
///The 60-bit signed integer type.
pub type i60 = Signed<60, i64>;
///The 61-bit signed integer type.
pub type i61 = Signed<61, i64>;
///The 62-bit signed integer type.
pub type i62 = Signed<62, i64>;
///The 63-bit signed integer type.
pub type i63 = Signed<63, i64>;

///The 65-bit signed integer type.
pub type i65 = Signed<65, i128>;
///The 66-bit signed integer type.
pub type i66 = Signed<66, i128>;
///The 67-bit signed integer type.
pub type i67 = Signed<67, i128>;
///The 68-bit signed integer type.
pub type i68 = Signed<68, i128>;
///The 69-bit signed integer type.
pub type i69 = Signed<69, i128>;
///The 70-bit signed integer type.
pub type i70 = Signed<70, i128>;
///The 71-bit signed integer type.
pub type i71 = Signed<71, i128>;
///The 72-bit signed integer type.
pub type i72 = Signed<72, i128>;
///The 73-bit signed integer type.
pub type i73 = Signed<73, i128>;
///The 74-bit signed integer type.
pub type i74 = Signed<74, i128>;
///The 75-bit signed integer type.
pub type i75 = Signed<75, i128>;
///The 76-bit signed integer type.
pub type i76 = Signed<76, i128>;
///The 77-bit signed integer type.
pub type i77 = Signed<77, i128>;
///The 78-bit signed integer type.
pub type i78 = Signed<78, i128>;
///The 79-bit signed integer type.
pub type i79 = Signed<79, i128>;
///The 80-bit signed integer type.
pub type i80 = Signed<80, i128>;
///The 81-bit signed integer type.
pub type i81 = Signed<81, i128>;
///The 82-bit signed integer type.
pub type i82 = Signed<82, i128>;
///The 83-bit signed integer type.
pub type i83 = Signed<83, i128>;
///The 84-bit signed integer type.
pub type i84 = Signed<84, i128>;
///The 85-bit signed integer type.
pub type i85 = Signed<85, i128>;
///The 86-bit signed integer type.
pub type i86 = Signed<86, i128>;
///The 87-bit signed integer type.
pub type i87 = Signed<87, i128>;
///The 88-bit signed integer type.
pub type i88 = Signed<88, i128>;
///The 89-bit signed integer type.
pub type i89 = Signed<89, i128>;
///The 90-bit signed integer type.
pub type i90 = Signed<90, i128>;
///The 91-bit signed integer type.
pub type i91 = Signed<91, i128>;
///The 92-bit signed integer type.
pub type i92 = Signed<92, i128>;
///The 93-bit signed integer type.
pub type i93 = Signed<93, i128>;
///The 94-bit signed integer type.
pub type i94 = Signed<94, i128>;
///The 95-bit signed integer type.
pub type i95 = Signed<95, i128>;
///The 96-bit signed integer type.
pub type i96 = Signed<96, i128>;
///The 97-bit signed integer type.
pub type i97 = Signed<97, i128>;
///The 98-bit signed integer type.
pub type i98 = Signed<98, i128>;
///The 99-bit signed integer type.
pub type i99 = Signed<99, i128>;
///The 100-bit signed integer type.
pub type i100 = Signed<100, i128>;
///The 101-bit signed integer type.
pub type i101 = Signed<101, i128>;
///The 102-bit signed integer type.
pub type i102 = Signed<102, i128>;
///The 103-bit signed integer type.
pub type i103 = Signed<103, i128>;
///The 104-bit signed integer type.
pub type i104 = Signed<104, i128>;
///The 105-bit signed integer type.
pub type i105 = Signed<105, i128>;
///The 106-bit signed integer type.
pub type i106 = Signed<106, i128>;
///The 107-bit signed integer type.
pub type i107 = Signed<107, i128>;
///The 108-bit signed integer type.
pub type i108 = Signed<108, i128>;
///The 109-bit signed integer type.
pub type i109 = Signed<109, i128>;
///The 110-bit signed integer type.
pub type i110 = Signed<110, i128>;
///The 111-bit signed integer type.
pub type i111 = Signed<111, i128>;
///The 112-bit signed integer type.
pub type i112 = Signed<112, i128>;
///The 113-bit signed integer type.
pub type i113 = Signed<113, i128>;
///The 114-bit signed integer type.
pub type i114 = Signed<114, i128>;
///The 115-bit signed integer type.
pub type i115 = Signed<115, i128>;
///The 116-bit signed integer type.
pub type i116 = Signed<116, i128>;
///The 117-bit signed integer type.
pub type i117 = Signed<117, i128>;
///The 118-bit signed integer type.
pub type i118 = Signed<118, i128>;
///The 119-bit signed integer type.
pub type i119 = Signed<119, i128>;
///The 120-bit signed integer type.
pub type i120 = Signed<120, i128>;
///The 121-bit signed integer type.
pub type i121 = Signed<121, i128>;
///The 122-bit signed integer type.
pub type i122 = Signed<122, i128>;
///The 123-bit signed integer type.
pub type i123 = Signed<123, i128>;
///The 124-bit signed integer type.
pub type i124 = Signed<124, i128>;
///The 125-bit signed integer type.
pub type i125 = Signed<125, i128>;
///The 126-bit signed integer type.
pub type i126 = Signed<126, i128>;
///The 127-bit signed integer type.
pub type i127 = Signed<127, i128>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_masking() {
        let new = |x| unsafe { u4::new_unchecked(x) };
        let newi = |x| unsafe { i4::new_unchecked(x) };
        assert_eq!(new(0b11000110).mask().0, 0b00000110);
        assert_eq!(new(0b00001000).mask().0, 0b00001000);
        assert_eq!(new(0b00001110).mask().0, 0b00001110);

        assert_eq!(newi(0b11000110u8 as i8).mask().0, 0b00000110u8 as i8);
        assert_eq!(newi(0b00001000u8 as i8).mask().0, 0b11111000u8 as i8);
        assert_eq!(newi(0b00001110u8 as i8).mask().0, 0b11111110u8 as i8);

    }

    #[test]
    fn min_max_values() {
        //assert_eq!(u1::MAX, u1::new(1));
        assert_eq!(u2::MAX, u2::new(3));
        assert_eq!(u3::MAX, u3::new(7));
        assert_eq!(u7::MAX, u7::new(127));
        assert_eq!(u9::MAX, u9::new(511));


        //assert_eq!(i1::MAX, i1::new(0));
        assert_eq!(i2::MAX, i2::new(1));
        assert_eq!(i3::MAX, i3::new(3));
        assert_eq!(i7::MAX, i7::new(63));
        assert_eq!(i9::MAX, i9::new(255));


        assert_eq!(u1::MIN, u1::new(0));
        assert_eq!(u2::MIN, u2::new(0));
        assert_eq!(u3::MIN, u3::new(0));
        assert_eq!(u7::MIN, u7::new(0));
        assert_eq!(u9::MIN, u9::new(0));
        assert_eq!(u127::MIN, u127::new(0));


        assert_eq!(i1::MIN, i1::new(-1));
        assert_eq!(i2::MIN, i2::new(-2));
        assert_eq!(i3::MIN, i3::new(-4));
        assert_eq!(i7::MIN, i7::new(-64));
        assert_eq!(i9::MIN, i9::new(-256));


    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(u1::MAX.wrapping_add(u1::new(1)), u1::new(0));
        assert_eq!(u1::MAX.wrapping_add(u1::new(0)), u1::new(1));

        assert_eq!(u5::MAX.wrapping_add(u5::new(1)), u5::new(0));
        assert_eq!(u5::MAX.wrapping_add(u5::new(4)), u5::new(3));

        assert_eq!(u127::MAX.wrapping_add(u127::new(100)), u127::new(99));
        assert_eq!(u127::MAX.wrapping_add(u127::new(1)), u127::new(0));

        assert_eq!(i1::MAX.wrapping_add(i1::new(0)), i1::new(0));
        assert_eq!(i1::MAX.wrapping_add(i1::new(-1)), i1::new(-1));

        assert_eq!(i7::MAX.wrapping_add(i7::new(1)), i7::MIN);
        assert_eq!(i7::MAX.wrapping_add(i7::new(4)), i7::new(-61));
    }

    #[test]
    #[should_panic]
    fn test_add_overflow_u5() {
        let _s = u5::MAX + u5::new(1);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow_u127() { let _s = u127::MAX + u127::new(1); }

    #[test]
    #[should_panic]
    fn test_add_overflow_i96() { let _s = i96::MAX + i96::new(100); }

    #[test]
    #[should_panic]
    fn test_add_underflow_i96() { let _s = i96::MIN + i96::new(-100); }

    #[test]
    #[should_panic]
    fn test_add_underflow_i17() {
        let _s = i17::MIN + i17::new(-1);
    }

    #[test]
    fn test_add() {
        assert_eq!(u5::new(1) + u5::new(2), u5::new(3));

        assert_eq!(i7::MAX + i7::MIN, i7::new(-1));
        assert_eq!(i7::new(4) + i7::new(-3), i7::new(1));
        assert_eq!(i7::new(-4) + i7::new(3), i7::new(-1));
        assert_eq!(i7::new(-3) + i7::new(-20), i7::new(-23));
    }

    #[test]
    #[should_panic]
    fn test_sub_overflow_i23() {
        let _s = i23::MIN - i23::MAX;
    }

    #[test]
    #[should_panic]
    fn test_sub_underflow_u5() {
        let _s = u5::MIN - u5::new(1);
    }

    #[test]
    #[should_panic]
    fn test_sub_underflow_i5() {
        let _s = i5::MIN - i5::new(1);
    }

    #[test]
    fn test_sub() {
        assert_eq!(u5::new(1) - u5::new(1), u5::new(0));
        assert_eq!(u5::new(3) - u5::new(2), u5::new(1));

        assert_eq!(i1::new(-1) - i1::new(-1) , i1::new(0));
        assert_eq!(i7::MIN - i7::MIN , i7::new(0));
        assert_eq!(i7::new(4) - i7::new(-3), i7::new(7));
        assert_eq!(i7::new(-4) - i7::new(3), i7::new(-7));
        assert_eq!(i7::new(-3) - i7::new(-20), i7::new(17));
    }

    #[test]
    fn test_shr() {
        assert_eq!(u5::new(8) >> 1usize, u5::new(4));
        assert_eq!(u5::new(8) >> 1u8, u5::new(4));
        assert_eq!(u5::new(8) >> 1u16, u5::new(4));
        assert_eq!(u5::new(8) >> 1u32, u5::new(4));
        assert_eq!(u5::new(8) >> 1u64, u5::new(4));
        assert_eq!(u5::new(8) >> 1isize, u5::new(4));
        assert_eq!(u5::new(8) >> 1i8, u5::new(4));
        assert_eq!(u5::new(8) >> 1i16, u5::new(4));
        assert_eq!(u5::new(8) >> 1i32, u5::new(4));
        assert_eq!(u5::new(8) >> 1i64, u5::new(4));

        assert_eq!(u5::MAX >> 4, u5::new(1));

        assert_eq!(i7::new(-1) >> 5, i7::new(-1));
    }

    #[test]
    fn test_shl() {
        let new = |x| unsafe { u5::new_unchecked(x) };
        let newi = |x| unsafe { i5::new_unchecked(x) };
        let newi2 = |x| unsafe { i7::new_unchecked(x) };
        assert_eq!(new(16) << 1usize, new(32));
        assert_eq!(new(16) << 1u8, new(32));
        assert_eq!(new(16) << 1u16, new(32));
        assert_eq!(new(16) << 1u32, new(32));
        assert_eq!(new(16) << 1u64, new(32));
        assert_eq!(new(16) << 1isize, new(32));
        assert_eq!(new(16) << 1i8, new(32));
        assert_eq!(new(16) << 1i16, new(32));
        assert_eq!(new(16) << 1i32, new(32));
        assert_eq!(new(16) << 1i64, new(32));

        assert_eq!(u5::MAX << 4, new(16));

        assert_eq!(newi(16) << 1, newi(0));
        assert_eq!(newi2(1) << 3, newi2(8));
    }

    #[test]
    fn test_shr_assign() {
        let mut x = u10::new(512);
        x >>= 1usize;
        assert_eq!(x, u10::new(256));
        x >>= 1isize;
        assert_eq!(x, u10::new(128));
        x >>= 1u8;
        assert_eq!(x, u10::new(64));
        x >>= 1i8;
        assert_eq!(x, u10::new(32));
        x >>= 2u64;
        assert_eq!(x, u10::new(8));
        x >>= 3i32;
        assert_eq!(x, u10::new(1));
    }

    #[test]
    fn test_shl_assign() {
        let mut x = u9::new(1);
        x <<= 3i32;
        assert_eq!(x, u9::new(8));
        x <<= 2u64;
        assert_eq!(x, u9::new(32));
        x <<= 1usize;
        assert_eq!(x, u9::new(64));
        x <<= 1isize;
        assert_eq!(x, u9::new(128));
        x <<= 1u8;
        assert_eq!(x, u9::new(256));
    }

    #[test]
    fn test_bitor() {
        assert_eq!(u9::new(1) | u9::new(8), u9::new(9));
        assert_eq!(&u9::new(1) | u9::new(8), u9::new(9));
        assert_eq!(u9::new(1) | &u9::new(8), u9::new(9));
        assert_eq!(&u9::new(1) | &u9::new(8), u9::new(9));
    }

    #[test]
    fn test_bitor_assign() {
        let mut x = u12::new(4);
        x |= u12::new(1);
        assert_eq!(x, u12::new(5));
        x |= u12::new(128);
        assert_eq!(x, u12::new(133));
        x = u12::new(1);
        x |= u12::new(127);
        assert_eq!(x, u12::new(127));
    }

    #[test]
    fn test_bitxor() {
        assert_eq!(u7::new(0x7F) ^ u7::new(42), u7::new(85));
        assert_eq!(&u7::new(0) ^ u7::new(42), u7::new(42));
        assert_eq!(u7::new(0x10) ^ &u7::new(0x1), u7::new(0x11));
        assert_eq!(&u7::new(11) ^ &u7::new(1), u7::new(10));
    }

    #[test]
    fn test_bitxor_assign() {
        let mut x = u12::new(4);
        x ^= u12::new(1);
        assert_eq!(x, u12::new(5));
        x ^= u12::new(128);
        assert_eq!(x, u12::new(133));
        x ^= u12::new(1);
        assert_eq!(x, u12::new(132));
        x ^= u12::new(127);
        assert_eq!(x, u12::new(251));
    }

    #[test]
    fn test_bitand() {
        //assert_eq!(i9::new(-7) & i9::new(-9), i9::from(-7i8 & -9i8));
        //assert_eq!(&i9::new(-7) & i9::new(-9), i9::from(&-7i8 & -9i8));
        //assert_eq!(i9::new(-7) & &i9::new(-9), i9::from(-7i8 & &-9i8));
        //assert_eq!(&i9::new(-7) & &i9::new(-9), i9::from(&-7i8 & &-9i8));

        assert_eq!(u9::new(8) & u9::new(9), u9::new(8));
        assert_eq!(&u9::new(8) & u9::new(9), u9::new(8));
        assert_eq!(u9::new(8) & &u9::new(9), u9::new(8));
        assert_eq!(&u9::new(8) & &u9::new(9), u9::new(8));
    }

    #[test]
    fn test_bitand_assign() {
        let mut x = u12::new(255);
        x &= u12::new(127);
        assert_eq!(x, u12::new(127));
        x &= u12::new(7);
        assert_eq!(x, u12::new(7));
        x &= u12::new(127);
        assert_eq!(x, u12::new(7));
        x &= u12::new(4);
        assert_eq!(x, u12::new(4));
    }

    #[test]
    fn test_not() {
        assert_eq!(!u7::new(42), u7::new(85));
        assert_eq!(!u7::new(0x7F), u7::new(0));
        assert_eq!(!u7::new(0), u7::new(0x7F));
        assert_eq!(!u7::new(56), u7::new(71));
    }
}
