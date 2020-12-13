use crate::buffer::math::matrix::Matrix;
use crate::buffer::math::vec3::Vec3;
use crate::buffer::math::vec4::Vec4;
use crate::buffer::math::vector::VecOps;
use crate::buffer::math::vector::Vector;

use std::f32;

pub type Mat4 = [f32; 16];

impl Matrix for Mat4 {
    type MatrixType = Mat4;
    type VectorType = Vec4;

    fn zeros() -> Self {
        [0.; 16]
    }
    fn ones() -> Self {
        [1.; 16]
    }
    fn identity() -> Self {
        [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ]
    }

    fn transpose(&mut self) -> &mut Self {
        let v01 = self[1];
        let v02 = self[2];
        let v03 = self[3];
        let v12 = self[6];
        let v13 = self[7];
        let v23 = self[11];

        self[1] = self[4];
        self[2] = self[8];
        self[3] = self[12];
        self[4] = v01;
        self[6] = self[9];
        self[7] = self[13];
        self[8] = v02;
        self[9] = v12;
        self[11] = self[14];
        self[12] = v03;
        self[13] = v13;
        self[14] = v23;

        self
    }

    fn mul(&mut self, rhs: &Self) -> Mat4 {
        let r00 = rhs[0];
        let r01 = rhs[1];
        let r02 = rhs[2];
        let r03 = rhs[3];
        let r10 = rhs[4];
        let r11 = rhs[5];
        let r12 = rhs[6];
        let r13 = rhs[7];
        let r20 = rhs[8];
        let r21 = rhs[9];
        let r22 = rhs[10];
        let r23 = rhs[11];
        let r30 = rhs[12];
        let r31 = rhs[13];
        let r32 = rhs[14];
        let r33 = rhs[15];

        let mut v0 = self[0];
        let mut v1 = self[1];
        let mut v2 = self[2];
        let mut v3 = self[3];

        let mut m: Mat4 = Mat4::zeros();
        m[0] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        m[1] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        m[2] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        m[3] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        v0 = self[4];
        v1 = self[5];
        v2 = self[6];
        v3 = self[7];
        m[4] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        m[5] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        m[6] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        m[7] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        v0 = self[8];
        v1 = self[9];
        v2 = self[10];
        v3 = self[11];
        m[8] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        m[9] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        m[10] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        m[11] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        v0 = self[12];
        v1 = self[13];
        v2 = self[14];
        v3 = self[15];
        m[12] = v0 * r00 + v1 * r10 + v2 * r20 + v3 * r30;
        m[13] = v0 * r01 + v1 * r11 + v2 * r21 + v3 * r31;
        m[14] = v0 * r02 + v1 * r12 + v2 * r22 + v3 * r32;
        m[15] = v0 * r03 + v1 * r13 + v2 * r23 + v3 * r33;

        m
    }

    fn add(&mut self, rhs: &Self) -> &mut Self {
        for i in 0..16 {
            self[i] += rhs[i];
        }

        self
    }
    fn sub(&mut self, rhs: &Self) -> &mut Self {
        for i in 0..16 {
            self[i] -= rhs[i];
        }

        self
    }

    fn scale(&mut self, factor: f32) -> &mut Self {
        self[0] *= factor;
        self[5] *= factor;
        self[10] *= factor;

        self
    }

    fn translate(&mut self, direction: &[f32]) -> &mut Self {
        debug_assert!(direction.len() >= 3);

        let mut x = direction[0];
        let mut y = direction[1];
        let mut z = direction[2];

        if direction.len() > 3 {
            x /= direction[3];
            y /= direction[3];
            z /= direction[3];
        }

        self[12] += self[0] * x + self[4] * y + self[8] * z;
        self[13] += self[1] * x + self[5] * y + self[9] * z;
        self[14] += self[2] * x + self[6] * y + self[10] * z;
        self[15] += self[3] * x + self[7] * y + self[11] * z;

        self
    }

    fn rotate(&mut self, angle: f32, axis: &[f32]) -> &mut Self {
        const EPSILON: f32 = 1e-5_f32;

        let mut x = axis[0];
        let mut y = axis[1];
        let mut z = axis[2];

        if axis.len() > 3 {
            x /= axis[3];
            y /= axis[3];
            z /= axis[3];
        }

        let len = (x * x + y * y + z * z).sqrt();

        if len.abs() <= EPSILON {
            debug_assert!(len.abs() > EPSILON);
            return self;
        }

        x /= len;
        y /= len;
        z /= len;

        let (s, c) = angle.sin_cos();
        let t = 1. - c;

        let v00 = self[0];
        let v01 = self[1];
        let v02 = self[2];
        let v03 = self[3];
        let v10 = self[4];
        let v11 = self[5];
        let v12 = self[6];
        let v13 = self[7];
        let v20 = self[8];
        let v21 = self[9];
        let v22 = self[10];
        let v23 = self[11];

        let rot00 = x * x * t + c;
        let rot01 = y * x * t + z * s;
        let rot02 = z * x * t - y * s;

        let rot10 = x * y * t - z * s;
        let rot11 = y * y * t + c;
        let rot12 = z * y * t + x * s;

        let rot20 = x * z * t + y * s;
        let rot21 = y * z * t - x * s;
        let rot22 = z * z * t + c;

        self[0] = v00 * rot00 + v10 * rot01 + v20 * rot02;
        self[1] = v01 * rot00 + v11 * rot01 + v21 * rot02;
        self[2] = v02 * rot00 + v12 * rot01 + v22 * rot02;
        self[3] = v03 * rot00 + v13 * rot01 + v23 * rot02;
        self[4] = v00 * rot10 + v10 * rot11 + v20 * rot12;
        self[5] = v01 * rot10 + v11 * rot11 + v21 * rot12;
        self[6] = v02 * rot10 + v12 * rot11 + v22 * rot12;
        self[7] = v03 * rot10 + v13 * rot11 + v23 * rot12;
        self[8] = v00 * rot20 + v10 * rot21 + v20 * rot22;
        self[9] = v01 * rot20 + v11 * rot21 + v21 * rot22;
        self[10] = v02 * rot20 + v12 * rot21 + v22 * rot22;
        self[11] = v03 * rot20 + v13 * rot21 + v23 * rot22;

        self
    }
}

pub trait ProjectionMatrix {
    fn create_perspective(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4;
}

impl ProjectionMatrix for Mat4 {
    fn create_perspective(mut fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        fov_y *= std::f32::consts::PI / 360.;
        let f = 1. / (fov_y).tan();
        let nf = 1. / (near - far);
        [
            f / aspect_ratio,
            0.,
            0.,
            0.,
            0.,
            f,
            0.,
            0.,
            0.,
            0.,
            (far + near) * nf,
            -1.,
            0.,
            0.,
            2. * far * near * nf,
            0.,
        ]
    }
}

pub trait WorldMatrix {
    fn set_lookat(eye: Vec3, center: Vec3, up: Vec3) -> Mat4;
}

impl WorldMatrix for Mat4 {
    fn set_lookat(eye: Vec3, center: Vec3, mut up: Vec3) -> Mat4 {
        let mut f: Vec3 = center;

        f = f.normalize(f);
        up = up.normalize(up);

        let s: Vec3 = f.cross(f, up);
        let u: Vec3 = s.cross(s, f);

        // let mut world2view: Mat4 = [
        //     s[0], u[0], -f[0], 0.,
        //     s[1], u[1], -f[1], 0.,
        //     s[2], u[2], -f[2], 0.,
        //     0., 0., 0., 1.,
        // ];
        let mut world2view: Mat4 = [
            s[0], s[1], s[2], 0., u[0], u[1], u[2], 0., -f[0], -f[1], -f[2], 0., 0., 0., 0., 1.,
        ];

        let m: Mat4 = [
            1., 0., 0., -eye[0], 0., 1., 0., -eye[1], 0., 0., 1., -eye[2], 0., 0., 0., 1.,
        ];

        world2view.mul(&m)
    }
}
