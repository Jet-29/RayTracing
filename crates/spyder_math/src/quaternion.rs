use std::ops::{Add, Index, Mul, Neg};

use crate::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub struct Quaternion<T: NumericFloat> {
    pub data: Vector<T, 4>,
}

impl<T: NumericFloat + std::fmt::Debug> Quaternion<T> {
    pub const IDENTITY: Self = Quaternion {
        data: Vector::<T, 4>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
    };

    pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self {
            data: (x, y, z, w).into(),
        }
    }

    pub fn from_angle_axis(angle: T, axis: Vector<T, 3>) -> Self {
        let axis = axis.normalise();
        let (s, c) = (angle * T::HALF).sin_cos_numeric();
        let v = axis * s;
        Self {
            data: Vector::<T, 4>::new(v[0], v[1], v[2], c),
        }
    }

    pub fn to_angle_axis(self) -> (T, Vector<T, 3>) {
        let v = Vector::<T, 3>::new(self.data[0], self.data[1], self.data[2]);
        let length = v.length();
        let axis = v / length;
        let angle = T::TWO * T::atan2(length, self.data[3]);
        (angle, axis)
    }

    pub fn as_array(self) -> [T; 4] {
        self.data.data[0]
    }

    pub fn from_yaw_pitch_roll(yaw: T, pitch: T, roll: T) -> Self {
        Self::from_angle_axis(yaw, <Vector<T, 3>>::Y)
            * Self::from_angle_axis(pitch, <Vector<T, 3>>::X)
            * Self::from_angle_axis(roll, <Vector<T, 3>>::Z)
    }

    pub fn rotate_vector3(&self, v: Vector<T, 3>) -> Vector<T, 3> {
        self.mul(v)
    }

    pub fn normalise(self) -> Self {
        Self {
            data: self.data.normalise(),
        }
    }

    pub fn from_forward_up(forward: Vector<T, 3>, up: Vector<T, 3>) -> Self {
        let looking_at_matrix =
            <Matrix<T, 4, 4>>::looking_at(<Vector<T, 3>>::ZERO, forward, up).inverse();
        looking_at_matrix.extract_rotation()
    }

    pub fn lerp(self, other: Self, amount: T) -> Self {
        Self {
            data: self.data + (other.data - self.data) * amount,
        }
        .normalise()
    }

    pub fn slerp(self, mut other: Self, amount: T) -> Self {
        let mut dot = self.data.dot(other.data);
        if dot < T::ZERO {
            other = other * -T::ONE;
            dot = -dot;
        };
        let dot_threshold = T::from_f32(0.9995);
        if dot > dot_threshold {
            self.lerp(other, amount)
        } else {
            let dot = dot.numeric_clamp(-T::ONE, T::ONE);
            let theta_0 = dot.acos();
            let theta = theta_0 * amount;
            let v2 = (other.data - self.data * dot).normalise();
            let (sin, cos) = theta.sin_cos_numeric();
            Self {
                data: self.data * cos + v2 * sin,
            }
        }
    }
}

impl<T: NumericFloat> Mul for Quaternion<T> {
    type Output = Self;
    fn mul(self, b: Self) -> Self::Output {
        let a = self.data;
        let b = b.data;
        Self {
            data: Vector::<T, 4>::new(
                a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
                a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
                a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
                a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
            ),
        }
    }
}

impl<T: NumericFloat> Mul<Vector<T, 3>> for Quaternion<T> {
    type Output = Vector<T, 3>;
    fn mul(self, other: Vector<T, 3>) -> Self::Output {
        let w = self.data[3];
        let b = Vector::<T, 3>::new(self.data[0], self.data[1], self.data[2]);
        let b2 = b.dot(b);
        other * (w * w - b2) + b * (other.dot(b) * T::TWO) + b.cross(other) * (w * T::TWO)
    }
}

impl<T: NumericFloat> Index<usize> for Quaternion<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data.data[0][index]
    }
}

impl<T: NumericFloat> From<Vector<T, 4>> for Quaternion<T> {
    fn from(value: Vector<T, 4>) -> Quaternion<T> {
        Self { data: value }
    }
}

impl<T: NumericFloat> From<(T, T, T, T)> for Quaternion<T> {
    fn from(value: (T, T, T, T)) -> Quaternion<T> {
        Self {
            data: [[value.0, value.1, value.2, value.3]].into(),
        }
    }
}

impl<T: NumericFloat> From<[T; 4]> for Quaternion<T> {
    fn from(value: [T; 4]) -> Quaternion<T> {
        Self { data: value.into() }
    }
}

impl<T: NumericFloat> From<Quaternion<T>> for [T; 4] {
    fn from(value: Quaternion<T>) -> [T; 4] {
        value.data.into()
    }
}

impl<T: NumericFloat> Add<Quaternion<T>> for Quaternion<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: self.data.add(other.data),
        }
    }
}

impl<T: NumericFloat> Mul<T> for Quaternion<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            data: self.data * other,
        }
    }
}

impl<T: NumericFloat> Neg for Quaternion<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            data: self.data * -T::ONE,
        }
    }
}
