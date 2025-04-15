fn celsius2fahrenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn fibonacci_loop(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

pub fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

// 👇 this is the key: test module inside same `mod calc`
#[cfg(test)]
mod calc_tests;
