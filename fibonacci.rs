fn fib(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}

pub fn main() {
    println!("{}", fib(5));
}
