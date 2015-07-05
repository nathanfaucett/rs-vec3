use num::Num;


#[inline(always)]
pub fn create<T: Num>(x: T, y: T, z: T) -> [T; 3] {[x, y, z]}
#[test]
fn test_create() {
    let v = create(1, 2, 3);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
    assert!(v[2] == 3);
}

#[inline(always)]
pub fn zero<T: Num>() -> [T; 3] {create(T::zero(), T::zero(), T::zero())}
#[inline(always)]
pub fn up<T: Num>() -> [T; 3] {create(T::zero(), T::zero(), T::one())}
#[inline(always)]
pub fn down<T: Num>() -> [T; 3] {create(T::zero(), T::zero(), -T::one())}
#[inline(always)]
pub fn forward<T: Num>() -> [T; 3] {create(T::zero(), T::one(), T::zero())}
#[inline(always)]
pub fn back<T: Num>() -> [T; 3] {create(T::zero(), -T::one(), -T::zero())}
#[inline(always)]
pub fn right<T: Num>() -> [T; 3] {create(T::one(), T::zero(), T::zero())}
#[inline(always)]
pub fn left<T: Num>() -> [T; 3] {create(-T::one(), T::zero(), T::zero())}

#[inline(always)]
pub fn clone<T: Num>(v: [T; 3]) -> [T; 3] {create(v[0], v[1], v[2])}

#[inline(always)]
pub fn copy<T: Num>(out: &mut [T; 3], a: [T; 3]) -> &mut [T; 3] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0, 0];
    copy(&mut v, [1, 2, 3]);
    assert!(v == [1, 2, 3]);
}
