use num::Num;


#[inline(always)]
pub fn new<'a, T: Num>(x: T, y: T, z: T) -> [T; 3] {[x, y, z]}
#[inline(always)]
pub fn create<'a, T: Num>(x: T, y: T, z: T) -> [T; 3] {new(x, y, z)}
#[test]
fn test_new() {
    let v = new(1, 2, 3);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
    assert!(v[2] == 3);
}

#[inline(always)]
pub fn clone<'a, T: Num>(v: &'a [T; 3]) -> [T; 3] {new(v[0], v[1], v[2])}

#[inline(always)]
pub fn copy<'a, T: Num>(out: &'a mut [T; 3], a: &'a [T; 3]) -> &'a mut [T; 3] {
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
