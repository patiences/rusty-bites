fn fact(n : i32) -> i32 {
    if n < 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

pub fn main() {
    println!("{}", fact(10));
}
