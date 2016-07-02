use num::Num;


#[inline(always)]
pub fn set<'a, 'b, T: Num>(out: &'a mut [T; 3], x: T, y: T, z: T) -> &'a mut [T; 3] {
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
pub fn zero<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, T::zero(), T::zero(), T::zero()) }
#[inline(always)]
pub fn identity<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, T::zero(), T::zero(), T::zero()) }
#[inline(always)]
pub fn up<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, T::zero(), T::zero(), T::one()) }
#[inline(always)]
pub fn down<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, T::zero(), T::zero(), -T::one()) }
#[inline(always)]
pub fn forward<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, T::zero(), T::one(), T::one()) }
#[inline(always)]
pub fn back<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, T::zero(), -T::one(), T::zero()) }
#[inline(always)]
pub fn left<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, -T::one(), T::zero(), T::one()) }
#[inline(always)]
pub fn right<'a, 'b, T: Num>(out: &'a mut [T; 3]) -> &'a mut [T; 3] { set(out, T::one(), T::zero(), T::one()) }
