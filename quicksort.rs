fn quicksort(v: &mut[i32]) {
    let length = v.len();
    quicksort_helper(v, 0, length-1);
}

fn quicksort_helper(v: &mut[i32], left: usize, right: usize) {
    let mut i = left; 
    let mut j = right; 
    let mut temp; 
    let pivot = v[(left + right) / 2];
    
    while i <= j {
        while v[i] < pivot {
            i = i + 1;
        }
        while v[j] > pivot {
            j = j - 1;
        }
        if i <= j {
            temp = v[i];
            v[i] = v[j];
            v[j] = temp; 
            i = i + 1; 
            j = j - 1; 
        }
    }
    
    if left < j {
        quicksort_helper(v, left, j); 
    } 
    if i < right {
        quicksort_helper(v, i, right);
    }
}

pub fn main() {
    let mut nums = vec![1, -43, 21, 2, 33, 5, 50, 100, 1000, 15, 9, 10001, -4];
    quicksort(nums.as_mut_slice());
    println!("{:?}", nums);
}
