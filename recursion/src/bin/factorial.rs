fn factorial(n: i64) -> i64 {
    if n < 2 {
        return 1;
    }

    n * factorial(n - 1)
}

fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n))
    }
}
