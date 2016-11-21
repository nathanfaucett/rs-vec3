use num::Unsigned;
use div::sdiv;


#[inline(always)]
pub fn length_values_sq<'a, 'b, T: Unsigned>(x: T, y: T, z: T) -> T {
    x * x + y * y + z * z
}
#[test]
fn test_length_values_sq() {
    assert!(length_values_sq(1, 1, 1) == 3);
}

#[inline(always)]
pub fn length_values<'a, 'b, T: Unsigned>(x: T, y: T, z: T) -> T {
    let lsq = length_values_sq(x, y, z);
    if lsq == T::zero() {lsq} else {lsq.sqrt()}
}
#[test]
fn test_length_values() {
    assert!(length_values(1, 2, 2) == 3);
}

#[inline(always)]
pub fn inv_length_values<'a, 'b, T: Unsigned>(x: T, y: T, z: T) -> T {
    let lsq = length_values_sq(x, y, z);
    if lsq == T::zero() {lsq} else {T::one() / lsq.sqrt()}
}
#[test]
fn test_inv_length_values() {
    assert!(inv_length_values(1.0, 2.0, 2.0) == 1.0 / 3.0);
}

#[inline(always)]
pub fn dot<'a, 'b, T: Unsigned>(a: &'b [T; 3], b: &'b [T; 3]) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[test]
fn test_dot() {
    assert!(dot(&[1, 1, 1], &[1, 1, 1]) == 3);
}

#[inline(always)]
pub fn cross<'a, 'b, T: Unsigned>(out: &'a mut [T; 3], a: &'b [T; 3], b: &'b [T; 3]) -> &'a mut [T; 3] {
    out[0] = a[1] * b[2] - a[2] * b[1];
    out[1] = a[2] * b[0] - a[0] * b[2];
    out[2] = a[0] * b[1] - a[1] * b[0];
    out
}
#[test]
fn test_cross() {
    let mut v = [0, 0, 0];
    cross(&mut v, &[1, 1, 1], &[1, 1, 1]);
    assert!(v[0] == 0);
    assert!(v[1] == 0);
    assert!(v[2] == 0);
}

#[inline(always)]
pub fn length<'a, 'b, T: Unsigned>(out: &'b [T; 3]) -> T {
    length_values(out[0], out[1], out[2])
}
#[test]
fn test_length() {
    assert!(length(&[1, 2, 2]) == 3);
}

#[inline(always)]
pub fn normalize<'a, 'b, T: Unsigned>(out: &'a mut [T; 3], a: &'b [T; 3]) -> &'a mut [T; 3] {
    sdiv(out, a, length(a))
}
#[test]
fn test_normalize() {
    let mut v = [0, 0, 0];
    normalize(&mut v, &[0, 0, 1]);
    assert!(v[0] == 0);
    assert!(v[1] == 0);
    assert!(v[2] == 1);
}
