use std::f32;

use crate::buffer::math::mat4::Mat4;
use crate::buffer::math::slice_ops::*;
use crate::buffer::math::vector::MulVectorMatrix;
use crate::buffer::math::vector::Vector;

pub type Vec4 = [f32; 4];

impl_vector!(Vec4, 4);

impl MulVectorMatrix<Mat4> for Vec4 {
    type VectorType = Vec4;

    fn mul_matrix_left(&self, lhs: &Mat4) -> Self::VectorType {
        let x = self[0];
        let y = self[1];
        let z = self[2];
        let w = self[3];

        [
            lhs[0] * x + lhs[1] * y + lhs[2] * z + lhs[3] * w,
            lhs[4] * x + lhs[5] * y + lhs[6] * z + lhs[7] * w,
            lhs[8] * x + lhs[9] * y + lhs[10] * z + lhs[11] * w,
            lhs[12] * x + lhs[13] * y + lhs[14] * z + lhs[15] * w,
        ]
    }

    fn mul_matrix(&self, rhs: &Mat4) -> Self::VectorType {
        let x = self[0];
        let y = self[1];
        let z = self[2];
        let w = self[3];

        [
            rhs[0] * x + rhs[4] * y + rhs[8] * z + rhs[12] * w,
            rhs[1] * x + rhs[5] * y + rhs[9] * z + rhs[13] * w,
            rhs[2] * x + rhs[6] * y + rhs[10] * z + rhs[14] * w,
            rhs[3] * x + rhs[7] * y + rhs[11] * z + rhs[15] * w,
        ]
    }
}
