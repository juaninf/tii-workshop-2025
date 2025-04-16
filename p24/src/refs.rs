fn f1(tuple: &mut (i32, i32, bool)) -> &mut i32 {
    if tuple.2 == false {
        &mut tuple.0
    } else {
        &mut tuple.1
    }
}

fn f2(x: &mut [u32], n: usize) -> &mut u32 {
    &mut x[n]
}

fn f3(x: &mut [u32], n: usize) -> &mut u32 {
    &mut x[x.len() - n - 1]
}

fn f4(x: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let x_len = x.len();
    let chunk_len = x_len / 4;
    (
        &x[0..chunk_len],
        &x[chunk_len..2 * chunk_len],
        &x[2 * chunk_len..3 * chunk_len],
        &x[3 * chunk_len..4 * chunk_len],
    )
}

#[cfg(test)]
mod ref_tests;
