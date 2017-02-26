use num::Num;


#[inline]
pub fn mul<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3], b: &'b [T; 3]) ->  &'a mut [T; 3] {
    out[0] = a[0] * b[0];
    out[1] = a[1] * b[1];
    out[2] = a[2] * b[2];
    out
}
#[test]
fn test_mul() {
    let mut v = [0, 0, 0];
    mul(&mut v, &[1, 1, 1], &[1, 1, 1]);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
}

#[inline]
pub fn smul<'a, 'b, T: Copy + Num>(out: &'a mut [T; 3], a: &'b [T; 3], s: T) ->  &'a mut [T; 3] {
    out[0] = a[0] * s;
    out[1] = a[1] * s;
    out[2] = a[2] * s;
    out
}
#[test]
fn test_smul() {
    let mut v = [0, 0, 0];
    smul(&mut v, &[1, 1, 1], 1);
    assert!(v[0] == 1);
    assert!(v[1] == 1);
    assert!(v[2] == 1);
}
