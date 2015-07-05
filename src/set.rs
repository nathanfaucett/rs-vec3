use num::Num;


#[inline(always)]
pub fn set<T: Num>(out: &mut [T; 3], x: T, y: T, z: T) -> &mut [T; 3] {
    out[0] = x;
    out[1] = y;
    out[2] = z;
    out
}
#[test]
fn test_set() {
    let mut v = [0, 0, 0];
    set(&mut v, 1, 2, 3);
    assert!(v == [1, 2, 3]);
}

#[inline(always)]
pub fn zero<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, T::zero(), T::zero(), T::zero()) }
#[inline(always)]
pub fn identity<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, T::zero(), T::zero(), T::zero()) }
#[inline(always)]
pub fn up<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, T::zero(), T::zero(), T::one()) }
#[inline(always)]
pub fn down<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, T::zero(), T::zero(), -T::one()) }
#[inline(always)]
pub fn forward<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, T::zero(), T::one(), T::one()) }
#[inline(always)]
pub fn back<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, T::zero(), -T::one(), T::zero()) }
#[inline(always)]
pub fn left<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, -T::one(), T::zero(), T::one()) }
#[inline(always)]
pub fn right<T: Num>(out: &mut [T; 3]) -> &mut [T; 3] { set(out, T::one(), T::zero(), T::one()) }
