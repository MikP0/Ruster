pub trait Vector {
    type VectorType;

    fn zeros() -> Self::VectorType;

    fn ones() -> Self::VectorType;

    fn mul(&self, rhs: &[f32]) -> Self::VectorType;

    fn add(&self, rhs: &[f32]) -> Self::VectorType;

    fn sub(&self, rhs: &[f32]) -> Self::VectorType;

    fn scale(&self, factor: f32) -> Self::VectorType;

    fn div(&self, factor: f32) -> Self::VectorType;
}

pub trait MulVectorMatrix<Matrix> {
    type VectorType;

    fn mul_matrix_left(&self, lhs: &Matrix) -> Self::VectorType;

    fn mul_matrix(&self, rhs: &Matrix) -> Self::VectorType;
}

pub trait VecOps {
    type VectorType;

    fn normalize(&self, vec: Self::VectorType) -> Self::VectorType;
    fn cross(&self, vec_a: Self::VectorType, vec_b: Self::VectorType) -> Self::VectorType;
}

macro_rules! impl_vector {
    ($type:ty, $n:expr) => {
        impl Vector for $type {
            type VectorType = $type;
            fn zeros() -> $type {
                [0.; $n]
            }

            fn ones() -> $type {
                [1.; $n]
            }

            fn mul(&self, rhs: &[f32]) -> $type {
                let mut dst = *self;
                mul(&mut dst, rhs);
                dst
            }

            fn add(&self, rhs: &[f32]) -> $type {
                let mut dst = *self;
                add(&mut dst, rhs);
                dst
            }

            fn sub(&self, rhs: &[f32]) -> $type {
                let mut dst = *self;
                sub(&mut dst, rhs);
                dst
            }

            fn scale(&self, factor: f32) -> $type {
                let mut dst = *self;
                scale(&mut dst, factor);
                dst
            }

            fn div(&self, factor: f32) -> $type {
                let mut dst = *self;
                div(&mut dst, factor);
                dst
            }
        }
    };
}
