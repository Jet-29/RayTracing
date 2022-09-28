use super::{Matrix, Quaternion, Vector};

pub type Vec2 = Vector<f32, 2>;
pub type Vec3 = Vector<f32, 3>;
pub type Vec4 = Vector<f32, 4>;

pub type Vec2d = Vector<f64, 2>;
pub type Vec3d = Vector<f64, 3>;
pub type Vec4d = Vector<f64, 4>;

pub type Vec2i = Vector<i32, 2>;
pub type Vec3i = Vector<i32, 3>;
pub type Vec4i = Vector<i32, 4>;

pub type Vec2u = Vector<usize, 2>;
pub type Vec3u = Vector<usize, 3>;
pub type Vec4u = Vector<usize, 4>;

pub type Mat3 = Matrix<f32, 3, 3>;
pub type Mat4 = Matrix<f32, 4, 4>;

pub type Quat = Quaternion<f32>;
pub type QuatDouble = Quaternion<f64>;
