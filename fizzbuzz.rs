fn fizzbuzz(n: usize) {
    for i in 1..n+1 {
        if i % 3 == 0 && i % 5 == 0{
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}

pub fn main() {
    fizzbuzz(100);
}
