use num::Num;


#[inline(always)]
pub fn inverse<T: Num>(out: &mut [T; 3], a: [T; 3]) -> &mut [T; 3] {
    out[0] = a[0] * -T::one();
    out[1] = a[1] * -T::one();
    out[2] = a[2] * -T::one();
    out
}
#[test]
fn test_inverse() {
    let mut v = [0, 0, 0];
    inverse(&mut v, [1, 1, 1]);
    assert!(v[0] == -1);
    assert!(v[1] == -1);
    assert!(v[2] == -1);
}

#[inline(always)]
pub fn lerp<T: Num>(out: &mut [T; 3], a: [T; 3], b: [T; 3], t: T) -> &mut [T; 3] {
    out[0] = a[0] + (b[0] - a[0]) * t;
    out[1] = a[1] + (b[1] - a[1]) * t;
    out[2] = a[2] + (b[2] - a[2]) * t;
    out
}
#[test]
fn test_lerp() {
    let mut v = [0.0, 0.0, 0.0];
    lerp(&mut v, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], 0.5);
    assert!(v[0] == 0.5);
    assert!(v[1] == 0.5);
    assert!(v[2] == 0.5);
}

#[inline(always)]
pub fn min<T: Num>(out: &mut [T; 3], a: [T; 3], b: [T; 3]) -> &mut [T; 3] {
    out[0] = if b[0] < a[0] {b[0]} else {a[0]};
    out[1] = if b[1] < a[1] {b[1]} else {a[1]};
    out[2] = if b[2] < a[2] {b[2]} else {a[2]};
    out
}
#[test]
fn test_min() {
    let mut v = [0, 0, 0];
    min(&mut v, [1, 0, 1], [0, 1, 0]);
    assert!(v == [0, 0, 0]);
}

#[inline(always)]
pub fn max<T: Num>(out: &mut [T; 3], a: [T; 3], b: [T; 3]) -> &mut [T; 3] {
    out[0] = if b[0] > a[0] {b[0]} else {a[0]};
    out[1] = if b[1] > a[1] {b[1]} else {a[1]};
    out[2] = if b[2] > a[2] {b[2]} else {a[2]};
    out
}
#[test]
fn test_max() {
    let mut v = [0, 0, 0];
    max(&mut v, [1, 0, 1], [0, 1, 0]);
    assert!(v == [1, 1, 1]);
}

#[inline(always)]
pub fn clamp<T: Num>(out: &mut [T; 3], a: [T; 3], min: [T; 3], max: [T; 3]) -> &mut [T; 3] {
    out[0] = if a[0] < min[0] {min[0]} else if a[0] > max[0] {max[0]} else {a[0]};
    out[1] = if a[1] < min[1] {min[1]} else if a[1] > max[1] {max[1]} else {a[1]};
    out[2] = if a[2] < min[2] {min[2]} else if a[2] > max[2] {max[2]} else {a[2]};
    out
}
#[test]
fn test_clamp() {
    let mut v = [0, 0, 0];
    clamp(&mut v, [2, 2, 2], [0, 0, 0], [1, 1, 1]);
    assert!(v == [1, 1, 1]);
}
