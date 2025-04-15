#![feature(test)]

use p22::calc::fibonacci_recursive;

extern crate test;
use test::bench;

#[bench]
fn bench_fib_recursive(b: &mut test::Bencher) {
    b.iter(|| {
        for i in 0..20 {
            fibonacci_recursive(i);
        }
    });
}
