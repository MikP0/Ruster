use crate::buffer::math::slice_ops::*;
use crate::buffer::math::vector::VecOps;
use crate::buffer::math::vector::Vector;

pub type Vec3 = [f32; 3];

impl_vector!(Vec3, 3);

impl VecOps for Vec3 {
    type VectorType = Vec3;

    fn normalize(&self, vec: Self::VectorType) -> Self::VectorType {
        let len = f32::sqrt(f32::powf(vec[0], 2.) + f32::powf(vec[1], 2.) + f32::powf(vec[2], 2.));
        [vec[0] / len, vec[1] / len, vec[2] / len]
    }

    fn cross(&self, vec_a: Self::VectorType, vec_b: Self::VectorType) -> Self::VectorType {
        [
            vec_a[1] * vec_b[2] - vec_a[2] * vec_b[1],
            -(vec_a[0] * vec_b[2] - vec_a[2] * vec_b[0]),
            vec_a[0] * vec_b[1] - vec_a[1] * vec_b[0],
        ]
    }
}
