fn linear_search(v: &[i32], key: i32) -> i32 {
    let mut index_of_key = -1;

    for i in 0..v.len() {
        if key == v[i] {
            index_of_key = i as i32;
        }
    }
    index_of_key
}

pub fn main() {
    let nums = vec![1, 2, 33, 4, 5, 50, 100, 1000, 10001];
    println!("{}", linear_search(nums.as_slice(), 100));
    println!("{}", linear_search(nums.as_slice(), 43859));
}
