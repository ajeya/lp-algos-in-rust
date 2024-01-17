use std::{
    io::{self, Write},
    usize,
};

use std::time::Instant;

// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i64>().expect("Error parsing integer")
}

fn fibonacci(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_bottom_up(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut n_1 = 0i64;
    let mut n_2 = 1i64;
    let mut fib = n_1 + n_2;

    for _ in 1i64..n + 1 {
        fib = n_1 + n_2;
        (n_1, n_2) = (fib, n_1);
    }
    fib
}

fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    if let Some(&value) = values.get(n as usize) {
        return value;
    }

    let value = fibonacci_on_the_fly(values, n - 1) + fibonacci_on_the_fly(values, n - 2);
    values.push(value);
    value
}

fn main() {
    // Initialize the prefilled vector.
    // let prefilled_values = prefill_vector();

    // Create a vector for fill-on-the-fly.
    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        let start = Instant::now();
        println!("Recursive fibonacci: {}", fibonacci(n));
        let duration = start.elapsed();
        println!("Recursive took {duration:?}");

        // Calculate the Fibonacci number.
        // println!("Prefilled:  {}", prefilled_values[n as usize]);
        let start = Instant::now();
        println!(
            "On the fly: {}",
            fibonacci_on_the_fly(&mut fill_on_the_fly_values, n)
        );
        let duration = start.elapsed();
        println!("Fill on the fly took {duration:?}");

        let start = Instant::now();
        println!("Bottom up:  {}", fibonacci_bottom_up(n));
        let duration = start.elapsed();
        println!("Bottom up took {duration:?}");
        println!();
    }
}
