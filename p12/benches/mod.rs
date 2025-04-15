#![feature(test)]

extern crate test;
use test::bench;

#[bench]
fn bench_add(b: &mut test::Bencher) {
    b.iter(|| {
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        sum
    });
}

fn bench_memcpy(b: &mut test::Bencher) {
    let src = vec![0u8; 1024];
    let mut dst = vec![0u8; 1024];
    b.iter(|| {
        dst.copy_from_slice(&src);
    });
}
