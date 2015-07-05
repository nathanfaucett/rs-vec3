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
