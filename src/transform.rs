use num::Num;
use signed::Signed;
use length::length_values;


#[inline]
pub fn transform_mat3<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3], m: &'b [T; 9]) -> &'a mut [T; 3] {
    out[0] = a[0] * m[0] + a[1] * m[3] + a[2] * m[6];
    out[1] = a[0] * m[1] + a[1] * m[4] + a[2] * m[7];
    out[2] = a[0] * m[2] + a[1] * m[5] + a[2] * m[8];
    out
}

#[inline]
pub fn transform_mat4<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3], m: &'b [T; 16]) -> &'a mut [T; 3] {
    out[0] = a[0] * m[0] + a[1] * m[4] + a[2] * m[8] + m[12];
    out[1] = a[0] * m[1] + a[1] * m[5] + a[2] * m[9] + m[13];
    out[2] = a[0] * m[2] + a[1] * m[6] + a[2] * m[10] + m[14];
    out
}

#[inline]
pub fn transform_mat4_rotation<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3], m: &'b [T; 16]) -> &'a mut [T; 3] {
    out[0] = a[0] * m[0] + a[1] * m[4] + a[2] * m[8];
    out[1] = a[0] * m[1] + a[1] * m[5] + a[2] * m[9];
    out[2] = a[0] * m[2] + a[1] * m[6] + a[2] * m[10];
    out
}

#[inline]
pub fn transform_projection<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3], m: &'b [T; 16]) -> &'a mut [T; 3] {
    let mut d = a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + m[15];
    d = if d != T::zero() {T::one() / d} else {d};

    out[0] = (a[0] * m[0] + a[1] * m[4] + a[2] * m[8] + m[12]) * d;
    out[1] = (a[0] * m[1] + a[1] * m[5] + a[2] * m[9] + m[13]) * d;
    out[2] = (a[0] * m[2] + a[1] * m[6] + a[2] * m[10] + m[14]) * d;
    out
}

#[inline]
pub fn transform_projection_no_position<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3], m: &'b [T; 16]) -> &'a mut [T; 3] {
    let mut d = a[0] * m[3] + a[1] * m[7] + a[2] * m[11] + m[15];
    d = if d != T::zero() {T::one() / d} else {d};

    out[0] = (a[0] * m[0] + a[1] * m[4] + a[2] * m[8]) * d;
    out[1] = (a[0] * m[1] + a[1] * m[5] + a[2] * m[9]) * d;
    out[2] = (a[0] * m[2] + a[1] * m[6] + a[2] * m[10]) * d;
    out
}

#[inline]
pub fn transform_quat<'a, 'b, T: Copy + Signed>(out: &'a mut [T; 3], a: &'b [T; 3], q: &'b [T; 4]) -> &'a mut [T; 3] {
    let ix = q[3] * a[0] + q[1] * a[2] - q[2] * a[1];
    let iy = q[3] * a[1] + q[2] * a[0] - q[0] * a[2];
    let iz = q[3] * a[2] + q[0] * a[1] - q[1] * a[0];
    let iw = -q[0] * a[0] - q[1] * a[1] - q[2] * a[2];

    out[0] = ix * q[3] + iw * -q[0] + iy * -q[2] - iz * -q[1];
    out[1] = iy * q[3] + iw * -q[1] + iz * -q[0] - ix * -q[2];
    out[2] = iz * q[3] + iw * -q[2] + ix * -q[1] - iy * -q[0];
    out
}

#[inline]
pub fn position_mat32<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], m: &'b [T; 6]) -> &'a mut [T; 3] {
    out[0] = m[4];
    out[1] = m[5];
    out
}

#[inline]
pub fn position_mat4<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], m: &'b [T; 16]) -> &'a mut [T; 3] {
    out[0] = m[12];
    out[1] = m[13];
    out[2] = m[14];
    out
}

#[inline]
pub fn scale_mat2<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], m: &'b [T; 4]) -> &'a mut [T; 3] {
    out[0] = length_values(m[0], m[2], T::zero());
    out[1] = length_values(m[1], m[3], T::zero());
    out[2] = T::one();
    out
}

#[inline]
pub fn scale_mat32<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], m: &'b [T; 6]) -> &'a mut [T; 3] {
    out[0] = length_values(m[0], m[2], T::zero());
    out[1] = length_values(m[1], m[3], T::zero());
    out[2] = T::one();
    out
}

#[inline]
pub fn scale_mat3<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], m: &'b [T; 9]) -> &'a mut [T; 3] {
    out[0] = length_values(m[0], m[3], m[6]);
    out[1] = length_values(m[1], m[4], m[7]);
    out[2] = length_values(m[2], m[5], m[8]);
    out
}

#[inline]
pub fn scale_mat4<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], m: &'b [T; 16]) -> &'a mut [T; 3] {
    out[0] = length_values(m[0], m[4], m[8]);
    out[1] = length_values(m[1], m[5], m[9]);
    out[2] = length_values(m[2], m[6], m[10]);
    out
}
