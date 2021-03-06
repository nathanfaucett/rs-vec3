use num::Num;


#[inline]
pub fn new<T: Copy + Num>(x: T, y: T, z: T) -> [T; 3] {[x, y, z]}
#[inline]
pub fn create<T: Copy + Num>(x: T, y: T, z: T) -> [T; 3] {new(x, y, z)}
#[test]
fn test_new() {
    let v = new(1, 2, 3);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
    assert!(v[2] == 3);
}

#[inline]
pub fn clone<'b, T: Copy + Num>(v: &'b [T; 3]) -> [T; 3] {new(v[0], v[1], v[2])}

#[inline]
pub fn copy<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3]) -> &'a mut [T; 3] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0, 0];
    copy(&mut v, &[1, 2, 3]);
    assert!(v == [1, 2, 3]);
}

#[inline]
pub fn from_vec2<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], v: &'b [T; 2]) -> &'a mut [T; 3] {
    out[0] = v[0];
    out[1] = v[1];
    out
}
#[inline]
pub fn from_vec4<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], v: &'b [T; 4]) -> &'a mut [T; 3] {
    out[0] = v[0];
    out[1] = v[1];
    out[2] = v[2];
    out
}
