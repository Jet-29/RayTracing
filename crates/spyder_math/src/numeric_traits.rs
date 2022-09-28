use std::{
    default::Default,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait Numeric:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
    + Default
    + Clone
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    const MAX: Self;
    const MIN: Self;

    fn numeric_min(self, other: Self) -> Self;
    fn numeric_max(self, other: Self) -> Self;
    fn numeric_clamp(self, min: Self, max: Self) -> Self;
}

pub trait NumericFloat:
    Numeric + NumericAbs + Neg<Output = Self> + PartialOrd + NumericSigNum + 'static
{
    const HALF: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const PI: Self;

    fn sin_cos_numeric(self) -> (Self, Self);
    fn tan_numeric(self) -> Self;
    fn is_nan_numeric(self) -> bool;
    fn copysign_numeric(self, sign: Self) -> Self;
    fn min_numeric(self, other: Self) -> Self;
    fn max_numeric(self, other: Self) -> Self;
    fn powf_numeric(self, other: Self) -> Self;
    fn floor_numeric(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn acos(self) -> Self;
    fn round_numeric(self) -> Self;
    fn from_f32(f: f32) -> Self;
    fn numeric_sqrt(self) -> Self;
    fn is_finite(self) -> bool;
}

pub trait NumericSigNum {
    fn signum_numeric(self) -> Self;
}

pub trait NumericAbs {
    fn numeric_abs(self) -> Self;
}

macro_rules! impl_numeric_float {
    ($($t:tt)*) => {
        $(
            impl NumericFloat for $t {
                const HALF: Self = 0.5;
                const INFINITY: Self = Self::INFINITY;
                const NEG_INFINITY: Self = Self::NEG_INFINITY;
                const PI: Self = std::$t::consts::PI;

                fn sin_cos_numeric(self) -> (Self, Self) {
                    self.sin_cos()
                }
                fn tan_numeric(self) -> Self {
                    self.tan()
                }
                fn is_nan_numeric(self) -> bool {
                    self.is_nan()
                }
                fn copysign_numeric(self, sign: Self) -> Self {
                    self.copysign(sign)
                }
                fn min_numeric(self, other: Self) -> Self {
                    self.min(other)
                }

                fn max_numeric(self, other: Self) -> Self {
                    self.max(other)
                }
                fn powf_numeric(self, other: Self) -> Self {
                    self.powf(other)
                }
                fn floor_numeric(self) -> Self {
                    self.floor()
                }
                fn atan2(self, other: Self) -> Self {
                    Self::atan2(self, other)
                }
                fn acos(self) -> Self {
                    Self::acos(self)
                }
                fn round_numeric(self) -> Self {
                    self.round()
                }
                fn from_f32(f: f32) -> Self {
                    f as $t
                }
                fn numeric_sqrt(self) -> Self {
                    self.sqrt()
                }
                fn is_finite(self) -> bool {
                    <core::primitive::$t>::is_finite(self)
                }
            }
        )*
    };
}

macro_rules! impl_numeric_signed {
    ($($t:tt)*) => {
        $(
            impl NumericAbs for $t {
                fn numeric_abs(self) -> Self {
                    self.abs()
                }
            }

            impl NumericSigNum for $t {
                fn signum_numeric(self) -> Self {
                    self.signum()
                }
            }
        )*
    };
}

macro_rules! impl_numeric {
    ($($t:tt)*) => {
        $(
            impl Numeric for $t {
                const ZERO: Self = 0 as $t;
                const ONE: Self = 1 as $t;
                const TWO: Self = 2 as $t;
                const MAX: Self = Self::MAX;
                const MIN: Self = Self::MIN;
                fn numeric_min(self, other: Self) -> Self {
                    self.min(other)
                }
                fn numeric_max(self, other: Self) -> Self {
                    self.max(other)
                }
                fn numeric_clamp(self, min: Self, max: Self) -> Self {
                    self.clamp(min, max)
                }
            }
        )*
    };
}

impl_numeric_float!(f32 f64);
impl_numeric_signed!(i8 i16 i32 i64 i128 isize f32 f64);
impl_numeric!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);
