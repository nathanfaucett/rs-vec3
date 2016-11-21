use num::Unsigned;


#[inline(always)]
pub fn sub<'a, 'b, T: Unsigned>(out: &'a mut [T; 3], a: &'b [T; 3], b: &'b [T; 3]) ->  &'a mut [T; 3] {
    out[0] = a[0] - b[0];
    out[1] = a[1] - b[1];
    out[2] = a[2] - b[2];
    out
}
#[test]
fn test_sub() {
    let mut v = [0, 0, 0];
    sub(&mut v, &[1, 1, 1], &[1, 1, 1]);
    assert!(v[0] == 0);
    assert!(v[1] == 0);
    assert!(v[2] == 0);
}

#[inline(always)]
pub fn ssub<'a, 'b, T: Unsigned>(out: &'a mut [T; 3], a: &'b [T; 3], s: T) ->  &'a mut [T; 3] {
    out[0] = a[0] - s;
    out[1] = a[1] - s;
    out[2] = a[2] - s;
    out
}
#[test]
fn test_ssub() {
    let mut v = [0, 0, 0];
    ssub(&mut v, &[1, 1, 1], 1);
    assert!(v[0] == 0);
    assert!(v[1] == 0);
    assert!(v[2] == 0);
}
