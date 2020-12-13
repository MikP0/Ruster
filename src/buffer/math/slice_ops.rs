use std::f32;

#[inline]
pub fn mul(lhs: &mut [f32], rhs: &[f32]) {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        *l *= r;
    }
}

#[inline]
pub fn add(lhs: &mut [f32], rhs: &[f32]) {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        *l += r;
    }
}

#[inline]
pub fn sub(lhs: &mut [f32], rhs: &[f32]) {
    for (l, r) in lhs.iter_mut().zip(rhs.iter()) {
        *l -= r;
    }
}

#[inline]
pub fn scale(seq: &mut [f32], factor: f32) {
    for i in seq.iter_mut() {
        *i *= factor;
    }
}

#[inline]
pub fn div(seq: &mut [f32], factor: f32) {
    for i in seq.iter_mut() {
        *i /= factor;
    }
}
