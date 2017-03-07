fn selection_sort(v: &mut[i32]) {
    let length : usize = v.len();
    let mut min_index: usize; 
    let mut temp: i32;

    for i in 0..length-1 {
        min_index = i;
        for j in i+1..length {
            if v[j] < v[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            temp = v[i];
            v[i] = v[min_index];
            v[min_index] = temp;
        }
    }
}


pub fn main() {
    let mut v : Vec<i32> = vec![1, 23, 13, 4, 6, -2, 18];
    selection_sort(v.as_mut_slice());
    println!("{:?}", v);
}
