use super::*;

#[test]
fn test_f1() {
    let mut tuple = (1, 2, false);
    assert_eq!(f1(&mut tuple), &mut 1);
    tuple.2 = true;
    assert_eq!(f1(&mut tuple), &mut 2);
}

#[test]
fn test_f2() {
    let mut x: [u32; 64] = [0; 64];
    //let x: &mut [u32] = &mut x;
    let n: usize = 32;
    assert_eq!(f2(&mut x, n), &mut 0);
}

#[test]
fn test_f3() {
    let mut x: [u32; 64] = [0; 64];
    //let x: &mut [u32] = &mut x;
    let n: usize = 32;
    assert_eq!(f3(&mut x, n), &mut 0);
}

#[test]
fn test_f4() {
    let mut x: [u32; 8] = [0; 8];
    //let x: &mut [u32] = &mut x;
    let (a, b, c, d) = f4(&mut x);
    assert_eq!(a, &[0; 2]);
    assert_eq!(b, &[0; 2]);
    assert_eq!(c, &[0; 2]);
    assert_eq!(d, &[0; 2]);
}
