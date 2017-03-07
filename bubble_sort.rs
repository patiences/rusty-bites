fn bubble_sort(v: &mut[i32]) {
    let length: usize = v.len();
    let mut temp: i32;
    
    for i in 0..length {
        for j in 1..length-i {
            if v[j-1] > v[j] {
                temp = v[j-1];
                v[j-1] = v[j];
                v[j] = temp;
            }
        }
        
    }
    
}


pub fn main() {
    let mut v : Vec<i32> = vec![1, 23, 13, 4, 6, -2, 18];
    bubble_sort(v.as_mut_slice());
    println!("{:?}", v);
}
