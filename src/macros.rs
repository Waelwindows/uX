use super::*;

use core::fmt::{Display, Octal, Binary, UpperHex, LowerHex, Formatter};
use core::cmp::{Ord, Ordering};
use core::convert::TryFrom;

macro_rules! impl_uint {
    ($type: ty) => {
        impl<const N: usize> Unsigned<N, $type> {
            pub const MAX: Self = Self((1 << N as $type)-1);
            pub const MIN: Self = Self(0);

            pub const NUM_MAX: $type = (1 << N as $type)-1;
            pub const NUM_MIN: $type = 0;

            pub(crate) const fn mask(self) -> Self {
                Self(self.0 & ( (1 as $type << N).overflowing_sub(1).0))
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
            /// assert_eq!(u31::new(64), u31::from(64u8));
            ///
            /// ```
            ///
            /// # Panic
            ///
            /// This function will panic if `value` is not representable by this type
            pub fn new(value: $type) -> Self {
                assert!(value <= Self::MAX.0 && value >= Self::MIN.0);
                unsafe { Self::new_unchecked(value) }
            }

            /// Crates a new variable while truncating the unused upper bits
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
            /// assert_eq!(u7::truncating_new(255), u7::new(127));
            ///
            /// ```
            pub const fn truncating_new(value: $type) -> Self {
                let num = unsafe { Self::new_unchecked(value) };
                num.mask()
            }

            pub const fn try_new(value: $type) -> Option<Self> {
                if value <= Self::MAX.0 && value >= Self::MIN.0 {
                    Some(unsafe { Self::new_unchecked(value) })
                } else {
                    None
                }
            }

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

        impl<const N: usize> PartialEq for Unsigned<N, $type> {
            fn eq(&self, other: &Self) -> bool {
                self.mask().0 == other.mask().0
            }
        }

        impl<const N: usize> Eq for Unsigned<N, $type> { }

        impl<const N: usize> PartialOrd for Unsigned<N, $type> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.mask().0.partial_cmp(&other.mask().0)
            }
        }

        impl<const N: usize> Ord for Unsigned<N, $type> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.mask().0.cmp(&other.mask().0)
            }
        }

        // Implement formating functions
        impl<const N: usize> Display for Unsigned<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Unsigned(ref value) = self;
                <$type as Display>::fmt(value, f)
            }
        }
        impl<const N: usize> UpperHex for Unsigned<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Unsigned(ref value) = self;
                <$type as UpperHex>::fmt(value, f)
            }
        }
        impl<const N: usize> LowerHex for Unsigned<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Unsigned(ref value) = self;
                <$type as LowerHex>::fmt(value, f)
            }
        }
        impl<const N: usize> Octal for Unsigned<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Unsigned(ref value) = self;
                <$type as Octal>::fmt(value, f)
            }
        }
        impl<const N: usize> Binary for Unsigned<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Unsigned(ref value) = self;
                <$type as Binary>::fmt(value, f)
            }
        }

        impl<T, const N: usize> Shl<T> for Unsigned<N, $type>
        where
            $type: Shl<T, Output=$type>
        {
            type Output = Self;

            fn shl(self, rhs: T) -> Self {
                Self(self.mask().0.shl(rhs))
            }
        }

        impl<T, const N: usize> ShlAssign<T> for Unsigned<N, $type>
        where
            $type: ShlAssign<T>
        {
            fn shl_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shl_assign(rhs);
            }
        }

        impl<T, const N: usize> Shr<T> for Unsigned<N, $type>
        where
            $type: Shr<T, Output=$type>
        {
            type Output = Self;

            fn shr(self, rhs: T) -> Self {
                Self(self.mask().0.shr(rhs))
            }
        }

        impl<T, const N: usize> ShrAssign<T> for Unsigned<N, $type>
        where
            $type: ShrAssign<T>
        {
            fn shr_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shr_assign(rhs);
            }
        }

        impl<const N: usize> BitOr<Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitOr<&'a Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = <Self as BitOr<Self>>::Output;

            fn bitor(self, rhs: &'a Unsigned<N, $type>) -> Self::Output {
                Self(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitOr<Unsigned<N, $type>> for &'a Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn bitor(self, rhs: Unsigned<N, $type>) -> Self::Output {
                Unsigned(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitOr<&'a Unsigned<N, $type>> for &'a Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn bitor(self, rhs: Self) -> Self::Output {
                Unsigned(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<const N: usize> BitOrAssign<Unsigned<N, $type>> for Unsigned<N, $type> {
            fn bitor_assign(&mut self, other: Unsigned<N, $type>) {
                *self = self.mask();
                self.0.bitor_assign(other.mask().0)
            }
        }

        impl<const N: usize> BitXor<Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn bitxor(self, rhs: Unsigned<N, $type>) -> Self::Output {
                Self(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitXor<&'a Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn bitxor(self, rhs: &'a Unsigned<N, $type>) -> Self::Output {
                Self(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitXor<Unsigned<N, $type>> for &'a Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn bitxor(self, rhs: Unsigned<N, $type>) -> Self::Output {
                Unsigned(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitXor<&'a Unsigned<N, $type>> for &'a Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn bitxor(self, rhs: &'a Unsigned<N, $type>) -> Self::Output {
                Unsigned(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<const N: usize> BitXorAssign<Unsigned<N, $type>> for Unsigned<N, $type> {
            fn bitxor_assign(&mut self, other: Unsigned<N, $type>) {
                *self = self.mask();
                self.0.bitxor_assign(other.mask().0)
            }
        }

        impl<const N: usize> Not for Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn not(self) -> Unsigned<N, $type> {
                Unsigned(self.mask().0.not())
            }
        }

        impl<'a, const N: usize> Not for &'a Unsigned<N, $type> {
            type Output = <Unsigned<N, $type> as Not>::Output;

            fn not(self) -> Unsigned<N, $type> {
                Unsigned(self.mask().0.not())
            }
        }

        impl<const N: usize> BitAnd<Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;

            fn bitand(self, rhs: Unsigned<N, $type>) -> Self::Output {
                Unsigned(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitAnd<&'a Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = <Unsigned<N, $type> as BitOr<Unsigned<N, $type>>>::Output;

            fn bitand(self, rhs: &'a Unsigned<N, $type>) -> Self::Output {
                Unsigned(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitAnd<Unsigned<N, $type>> for &'a Unsigned<N, $type> {
            type Output = <Unsigned<N, $type> as BitOr<Unsigned<N, $type>>>::Output;

            fn bitand(self, rhs: Unsigned<N, $type>) -> Self::Output {
                Unsigned(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitAnd<&'a Unsigned<N, $type>> for &'a Unsigned<N, $type> {
            type Output = <Unsigned<N, $type> as BitOr<Unsigned<N, $type>>>::Output;

            fn bitand(self, rhs: &'a Unsigned<N, $type>) -> Self::Output {
                Unsigned(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<const N: usize> BitAndAssign<Unsigned<N, $type>> for Unsigned<N, $type> {
            fn bitand_assign(&mut self, other: Unsigned<N, $type>) {
                *self = self.mask();
                self.0.bitand_assign(other.mask().0)
            }
        }

        impl<const N: usize> lib::core::ops::Add<Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;
            #[allow(unused_comparisons)]
            fn add(self, other: Unsigned<N, $type>) -> Unsigned<N, $type> {
                if self.0 > 0 && other.0 > 0 {
                    debug_assert!(Self::MAX.0 - other.0 >= self.0);
                } else if self.0 < 0 && other.0 < 0 {
                    debug_assert!(Self::MIN.0 - other.0 <= self.0);
                }
                self.wrapping_add(other)
            }
        }

        impl<const N: usize> lib::core::ops::Sub<Unsigned<N, $type>> for Unsigned<N, $type> {
            type Output = Unsigned<N, $type>;
            #[allow(unused_comparisons)]
            fn sub(self, other: Unsigned<N, $type>) -> Unsigned<N, $type> {
                if self > other {
                    debug_assert!(Self::MAX.0 + other.0 >= self.0);
                } else if self < other {
                    debug_assert!(Self::MIN.0 + other.0 <= self.0);
                }
                self.wrapping_sub(other)
            }
        }

        impl<const N: usize, T: Into<$type>> From<T> for Unsigned<N, $type> {
            fn from(num: T) -> Self {
                Self::new(num.into())
            }
        }

        //impl<const N: usize> TryFrom<$type> for Unsigned<N, $type> {
            //type Error = OverflowError;

            //fn try_from(num: $type) -> Result<Self, Self::Error> {
                //Self::try_new(num).ok_or(OverflowError)
            //}
        //}
    };
}

macro_rules! impl_int {
    ($type: ty) => {
        impl<const N: usize> Signed<N, $type> {
            pub const MAX: Self = Self((1 << N-1)-1);
            pub const MIN: Self = Self(-(1<<N-1));

            pub(crate) const fn mask(self) -> Self {
                if ( self.0 & (1<<(N-1)) ) == 0 {
                    Self(self.0 & ( ((1 as $type) << N).overflowing_sub(1).0))
                } else {
                    Self(self.0 | !( ((1 as $type) << N).overflowing_sub(1).0))
                }
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
            /// assert_eq!(u31::new(64), u31::from(64u8));
            ///
            /// ```
            ///
            /// # Panic
            ///
            /// This function will panic if `value` is not representable by this type
            pub fn new(value: $type) -> Self {
                assert!(value <= Self::MAX.0 && value >= Self::MIN.0);
                unsafe { Self::new_unchecked(value) }
            }

            /// Crates a new variable while truncating the unused upper bits
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
            /// assert_eq!(i7::truncating_new(127), i7::new(63));
            /// assert_eq!(i7::truncating_new(127), i7::new(63));
            ///
            /// ```
            pub const fn truncating_new(value: $type) -> Self {
                let num = unsafe { Self::new_unchecked(value) };
                num.mask()
            }

            pub const fn try_new(value: $type) -> Option<Self> {
                if value <= Self::MAX.0 && value >= Self::MIN.0 {
                    Some(unsafe { Self::new_unchecked(value) })
                } else {
                    None
                }
            }

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

        impl<const N: usize> PartialEq for Signed<N, $type> {
            fn eq(&self, other: &Self) -> bool {
                self.mask().0 == other.mask().0
            }
        }

        impl<const N: usize> Eq for Signed<N, $type> { }

        impl<const N: usize> PartialOrd for Signed<N, $type> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.mask().0.partial_cmp(&other.mask().0)
            }
        }

        impl<const N: usize> Ord for Signed<N, $type> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.mask().0.cmp(&other.mask().0)
            }
        }

        // Implement formating functions
        impl<const N: usize> Display for Signed<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Signed(ref value) = self;
                <$type as Display>::fmt(value, f)
            }
        }
        impl<const N: usize> UpperHex for Signed<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Signed(ref value) = self;
                <$type as UpperHex>::fmt(value, f)
            }
        }
        impl<const N: usize> LowerHex for Signed<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Signed(ref value) = self;
                <$type as LowerHex>::fmt(value, f)
            }
        }
        impl<const N: usize> Octal for Signed<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Signed(ref value) = self;
                <$type as Octal>::fmt(value, f)
            }
        }
        impl<const N: usize> Binary for Signed<N, $type> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), lib::core::fmt::Error> {
                let &Signed(ref value) = self;
                <$type as Binary>::fmt(value, f)
            }
        }

        impl<T, const N: usize> Shl<T> for Signed<N, $type>
        where
            $type: Shl<T, Output=$type>
        {
            type Output = Self;

            fn shl(self, rhs: T) -> Self {
                Self(self.mask().0.shl(rhs))
            }
        }

        impl<T, const N: usize> ShlAssign<T> for Signed<N, $type>
        where
            $type: ShlAssign<T>
        {
            fn shl_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shl_assign(rhs);
            }
        }

        impl<T, const N: usize> Shr<T> for Signed<N, $type>
        where
            $type: Shr<T, Output=$type>
        {
            type Output = Self;

            fn shr(self, rhs: T) -> Self {
                Self(self.mask().0.shr(rhs))
            }
        }

        impl<T, const N: usize> ShrAssign<T> for Signed<N, $type>
        where
            $type: ShrAssign<T>
        {
            fn shr_assign(&mut self, rhs: T) {
                *self = self.mask();
                self.0.shr_assign(rhs);
            }
        }

        impl<const N: usize> BitOr<Signed<N, $type>> for Signed<N, $type> {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitOr<&'a Signed<N, $type>> for Signed<N, $type> {
            type Output = <Self as BitOr<Self>>::Output;

            fn bitor(self, rhs: &'a Signed<N, $type>) -> Self::Output {
                Self(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitOr<Signed<N, $type>> for &'a Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn bitor(self, rhs: Signed<N, $type>) -> Self::Output {
                Signed(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitOr<&'a Signed<N, $type>> for &'a Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn bitor(self, rhs: Self) -> Self::Output {
                Signed(self.mask().0.bitor(rhs.mask().0))
            }
        }

        impl<const N: usize> BitOrAssign<Signed<N, $type>> for Signed<N, $type> {
            fn bitor_assign(&mut self, other: Signed<N, $type>) {
                *self = self.mask();
                self.0.bitor_assign(other.mask().0)
            }
        }

        impl<const N: usize> BitXor<Signed<N, $type>> for Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn bitxor(self, rhs: Signed<N, $type>) -> Self::Output {
                Self(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitXor<&'a Signed<N, $type>> for Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn bitxor(self, rhs: &'a Signed<N, $type>) -> Self::Output {
                Self(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitXor<Signed<N, $type>> for &'a Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn bitxor(self, rhs: Signed<N, $type>) -> Self::Output {
                Signed(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitXor<&'a Signed<N, $type>> for &'a Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn bitxor(self, rhs: &'a Signed<N, $type>) -> Self::Output {
                Signed(self.mask().0.bitxor(rhs.mask().0))
            }
        }

        impl<const N: usize> BitXorAssign<Signed<N, $type>> for Signed<N, $type> {
            fn bitxor_assign(&mut self, other: Signed<N, $type>) {
                *self = self.mask();
                self.0.bitxor_assign(other.mask().0)
            }
        }

        impl<const N: usize> Not for Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn not(self) -> Signed<N, $type> {
                Signed(self.mask().0.not())
            }
        }

        impl<'a, const N: usize> Not for &'a Signed<N, $type> {
            type Output = <Signed<N, $type> as Not>::Output;

            fn not(self) -> Signed<N, $type> {
                Signed(self.mask().0.not())
            }
        }

        impl<const N: usize> BitAnd<Signed<N, $type>> for Signed<N, $type> {
            type Output = Signed<N, $type>;

            fn bitand(self, rhs: Signed<N, $type>) -> Self::Output {
                Signed(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitAnd<&'a Signed<N, $type>> for Signed<N, $type> {
            type Output = <Signed<N, $type> as BitOr<Signed<N, $type>>>::Output;

            fn bitand(self, rhs: &'a Signed<N, $type>) -> Self::Output {
                Signed(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitAnd<Signed<N, $type>> for &'a Signed<N, $type> {
            type Output = <Signed<N, $type> as BitOr<Signed<N, $type>>>::Output;

            fn bitand(self, rhs: Signed<N, $type>) -> Self::Output {
                Signed(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<'a, const N: usize> BitAnd<&'a Signed<N, $type>> for &'a Signed<N, $type> {
            type Output = <Signed<N, $type> as BitOr<Signed<N, $type>>>::Output;

            fn bitand(self, rhs: &'a Signed<N, $type>) -> Self::Output {
                Signed(self.mask().0.bitand(rhs.mask().0))
            }
        }

        impl<const N: usize> BitAndAssign<Signed<N, $type>> for Signed<N, $type> {
            fn bitand_assign(&mut self, other: Signed<N, $type>) {
                *self = self.mask();
                self.0.bitand_assign(other.mask().0)
            }
        }

        impl<const N: usize> lib::core::ops::Add<Signed<N, $type>> for Signed<N, $type> {
            type Output = Signed<N, $type>;
            #[allow(unused_comparisons)]
            fn add(self, other: Signed<N, $type>) -> Signed<N, $type> {
                if self.0 > 0 && other.0 > 0 {
                    debug_assert!(Self::MAX.0 - other.0 >= self.0);
                } else if self.0 < 0 && other.0 < 0 {
                    debug_assert!(Self::MIN.0 - other.0 <= self.0);
                }
                self.wrapping_add(other)
            }
        }

        impl<const N: usize> lib::core::ops::Sub<Signed<N, $type>> for Signed<N, $type> {
            type Output = Signed<N, $type>;
            #[allow(unused_comparisons)]
            fn sub(self, other: Signed<N, $type>) -> Signed<N, $type> {
                if self > other {
                    debug_assert!(Self::MAX.0 + other.0 >= self.0);
                } else if self < other {
                    debug_assert!(Self::MIN.0 + other.0 <= self.0);
                }
                self.wrapping_sub(other)
            }
        }
        
        impl<const N: usize> From<Signed<N, $type>> for $type {
            fn from(num: Signed<N, $type>) -> $type {
                num.0
            }
        }

        impl<const N: usize> TryFrom<$type> for Signed<N, $type> {
            type Error = OverflowError;

            fn try_from(num: $type) -> Result<Self, Self::Error> {
                Self::try_new(num).ok_or(OverflowError)
            }
        }
    };
}

pub struct OverflowError;

impl_uint!(u8);
impl_uint!(u16);
impl_uint!(u32);
impl_uint!(u64);
impl_uint!(u128);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
