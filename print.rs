fn print_ints(v: &[i32]) {
    for x in v {
        println!("{}", x);
    }
}

fn print_chars(v: &[char]) {
    for x in v {
        println!("{}", x);
    }
}

pub fn main() { 
    let nums : Vec<i32> = vec![1, 2, 3, 4, 5];
    print_ints(nums.as_slice());
    
    let alphabet = vec!['a', 'b','c', 'd', 'e'];
    print_chars(alphabet.as_slice());
}
