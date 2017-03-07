fn fact(n : i32) -> i32 {
    if n < 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn fact_smart(n: i32) -> i32 {
    (1..n).fold(1, |x, y| x * y)
}

pub fn main() {
    println!("{}", fact(10));
    println!("{}", fact_smart(10));
}
