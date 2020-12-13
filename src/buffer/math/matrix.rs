pub trait Matrix {
    type MatrixType;
    type VectorType;

    fn zeros() -> Self::MatrixType;

    fn ones() -> Self::MatrixType;

    fn identity() -> Self::MatrixType;

    fn transpose(&mut self) -> &mut Self::MatrixType;

    fn mul(&mut self, rhs: &Self::MatrixType) -> Self::MatrixType;

    fn add(&mut self, rhs: &Self::MatrixType) -> &mut Self::MatrixType;

    fn sub(&mut self, rhs: &Self::MatrixType) -> &mut Self::MatrixType;

    fn scale(&mut self, factor: f32) -> &mut Self::MatrixType;

    fn translate(&mut self, direction: &[f32]) -> &mut Self::MatrixType;

    fn rotate(&mut self, angle: f32, axis: &[f32]) -> &mut Self::MatrixType;
}
