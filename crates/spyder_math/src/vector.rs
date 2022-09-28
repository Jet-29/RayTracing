use std::{
    ops::{Deref, DerefMut},
    usize,
};

use crate::*;

pub type Vector<T, const N: usize> = Matrix<T, N, 1>;

impl<T, const N: usize> Vector<T, N> {
    pub const fn new_from_slice(values: [T; N]) -> Self {
        Self { data: [values] }
    }
}

impl<T> Vector<T, 2> {
    pub const fn new(x: T, y: T) -> Self {
        Self { data: [[x, y]] }
    }
}

impl<T> Vector<T, 3> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { data: [[x, y, z]] }
    }
}

impl<T> Vector<T, 4> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {
            data: [[x, y, z, w]],
        }
    }
}

impl<T> From<T> for Vector<T, 1> {
    fn from(value: T) -> Self {
        Self { data: [[value]] }
    }
}

impl<T> From<(T,)> for Vector<T, 1> {
    fn from(value: (T,)) -> Self {
        Self { data: [[value.0]] }
    }
}

impl<T> From<(T, T)> for Vector<T, 2> {
    fn from(value: (T, T)) -> Self {
        Self {
            data: [[value.0, value.1]],
        }
    }
}

impl<T: Copy> From<(T, T, T)> for Vector<T, 3> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            data: [[value.0, value.1, value.2]],
        }
    }
}

impl<T: Copy> From<(T, T, T, T)> for Vector<T, 4> {
    fn from(value: (T, T, T, T)) -> Vector<T, 4> {
        Self {
            data: [[value.0, value.1, value.2, value.3]],
        }
    }
}

impl<T: Copy> From<Vector<T, 1>> for (T,) {
    fn from(value: Vector<T, 1>) -> (T,) {
        (value.data[0][0],)
    }
}

impl<T: Copy> From<Vector<T, 2>> for (T, T) {
    fn from(value: Vector<T, 2>) -> (T, T) {
        (value.data[0][0], value.data[0][1])
    }
}

impl<T: Copy> From<Vector<T, 3>> for (T, T, T) {
    fn from(value: Vector<T, 3>) -> (T, T, T) {
        (value.data[0][0], value.data[0][1], value.data[0][2])
    }
}

impl<T: Copy> From<Vector<T, 4>> for (T, T, T, T) {
    fn from(value: Vector<T, 4>) -> (T, T, T, T) {
        (
            value.data[0][0],
            value.data[0][1],
            value.data[0][2],
            value.data[0][3],
        )
    }
}

impl<T: Numeric, const N: usize> Vector<T, N> {
    pub fn as_array(self) -> [T; N] {
        self.data[0]
    }

    #[inline]
    pub fn dot(self, other: Self) -> T {
        let mut total = T::ZERO;
        for i in 0..N {
            total = total + self.data[0][i] * other.data[0][i];
        }
        total
    }

    pub fn xy(&self) -> Vector<T, 2> {
        Vector::<T, 2>::new(
            self.data[0][0],
            if N > 1 { self.data[0][1] } else { T::ZERO },
        )
    }

    pub fn xz(&self) -> Vector<T, 2> {
        Vector::<T, 2>::new(
            self.data[0][0],
            if N > 2 { self.data[0][2] } else { T::ZERO },
        )
    }

    pub fn xyz(&self) -> Vector<T, 3> {
        Vector::<T, 3>::new(
            self.data[0][0],
            if N > 1 { self.data[0][1] } else { T::ZERO },
            if N > 2 { self.data[0][2] } else { T::ZERO },
        )
    }

    pub fn xyzw(&self) -> Vector<T, 4> {
        Vector::<T, 4>::new(
            self.data[0][0],
            if N > 1 { self.data[0][1] } else { T::ZERO },
            if N > 2 { self.data[0][2] } else { T::ZERO },
            if N > 3 { self.data[0][2] } else { T::ZERO },
        )
    }

    pub fn zxy(&self) -> Vector<T, 3> {
        Vector::<T, 3>::new(
            if N > 2 { self.data[0][2] } else { T::ZERO },
            self.data[0][0],
            if N > 1 { self.data[0][1] } else { T::ZERO },
        )
    }
}

pub trait Extend<T> {
    type ExtendTo;
    fn extend(self, value: T) -> Self::ExtendTo;
}

impl<T: Numeric> Extend<T> for Vector<T, 1> {
    type ExtendTo = Vector<T, 2>;
    fn extend(self, y: T) -> Self::ExtendTo {
        Vector::<T, 2>::new(self.data[0][0], y)
    }
}

impl<T: Numeric> Extend<T> for Vector<T, 2> {
    type ExtendTo = Vector<T, 3>;
    fn extend(self, z: T) -> Self::ExtendTo {
        Vector::<T, 3>::new(self.data[0][0], self.data[0][1], z)
    }
}

impl<T: Numeric> Extend<T> for Vector<T, 3> {
    type ExtendTo = Vector<T, 4>;
    fn extend(self, w: T) -> Self::ExtendTo {
        Vector::<T, 4>::new(self.data[0][0], self.data[0][1], self.data[0][2], w)
    }
}

impl<T: Numeric> Vector<T, 1> {
    pub fn x(self) -> T {
        self.data[0][0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.data[0][0]
    }
}

impl<T: Numeric> Vector<T, 2> {
    pub fn x(self) -> T {
        self.data[0][0]
    }

    pub fn y(self) -> T {
        self.data[0][1]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.data[0][0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.data[0][1]
    }
}

impl<T: Numeric> Vector<T, 3> {
    pub fn x(self) -> T {
        self.data[0][0]
    }

    pub fn y(self) -> T {
        self.data[0][1]
    }

    pub fn z(self) -> T {
        self.data[0][2]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.data[0][0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.data[0][1]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.data[0][2]
    }
}

impl<T: Numeric> Vector<T, 4> {
    pub fn x(self) -> T {
        self.data[0][0]
    }

    pub fn y(self) -> T {
        self.data[0][1]
    }

    pub fn z(self) -> T {
        self.data[0][2]
    }

    pub fn w(self) -> T {
        self.data[0][3]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.data[0][0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.data[0][1]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.data[0][2]
    }

    pub fn w_mut(&mut self) -> &mut T {
        &mut self.data[0][3]
    }
}

impl<T: NumericFloat, const N: usize> Vector<T, N> {
    pub fn length(self) -> T {
        self.dot(self).numeric_sqrt()
    }

    pub fn length_squared(self) -> T {
        self.dot(self)
    }

    pub fn normalise(self) -> Self {
        self / self.dot(self).numeric_sqrt()
    }

    pub fn normalise_or_zero(self) -> Self {
        let one_div_length = T::ONE / self.dot(self).numeric_sqrt();
        if one_div_length.is_finite() {
            self * one_div_length
        } else {
            Self::ZERO
        }
    }
}

impl<T: Numeric, const N: usize> Vector<T, N> {
    pub const X: Self = {
        let mut v = Self::ZERO;
        if N >= 1 {
            v.data[0][0] = T::ONE;
        }
        v
    };
    pub const Y: Self = {
        let mut v = Self::ZERO;
        if N >= 2 {
            v.data[0][1] = T::ONE;
        }
        v
    };
    pub const Z: Self = {
        let mut v = Self::ZERO;
        if N >= 3 {
            v.data[0][2] = T::ONE;
        }
        v
    };
    pub const W: Self = {
        let mut v = Self::ZERO;
        if N >= 4 {
            v.data[0][3] = T::ONE;
        }
        v
    };

    pub const XY: Self = {
        let mut v = Self::ZERO;
        if N >= 1 {
            v.data[0][0] = T::ONE;
        }
        if N >= 2 {
            v.data[0][1] = T::ONE;
        }
        v
    };
    pub const XZ: Self = {
        let mut v = Self::ZERO;
        if N >= 1 {
            v.data[0][0] = T::ONE;
        }
        if N >= 3 {
            v.data[0][2] = T::ONE;
        }
        v
    };
    pub const YZ: Self = {
        let mut v = Self::ZERO;
        if N >= 2 {
            v.data[0][1] = T::ONE;
        }
        if N >= 3 {
            v.data[0][2] = T::ONE;
        }
        v
    };
}

impl<T: NumericFloat> Vector<T, 3> {
    pub fn cross(self, other: Self) -> Self {
        (self.zxy().mul_by_component(other) - self.mul_by_component(other.zxy())).zxy()
    }

    pub fn counter_clockwise_angle_between(self, other: Self, axis: Self) -> T {
        let v = self.cross(other).dot(axis);
        let dot = self.dot(other);
        T::atan2(-v, -dot) + T::PI
    }
}

#[repr(C)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
pub struct XYZ<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
pub struct XYZW<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Deref for Vector<T, 2> {
    type Target = XY<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.data.as_ptr() as *const _) }
    }
}

impl<T> DerefMut for Vector<T, 2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.data.as_mut_ptr() as *mut _) }
    }
}

impl<T> Deref for Vector<T, 3> {
    type Target = XYZ<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.data.as_ptr() as *const _) }
    }
}

impl<T> DerefMut for Vector<T, 3> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.data.as_mut_ptr() as *mut _) }
    }
}

impl<T> Deref for Vector<T, 4> {
    type Target = XYZW<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.data.as_ptr() as *const _) }
    }
}

impl<T> DerefMut for Vector<T, 4> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.data.as_mut_ptr() as *mut _) }
    }
}
