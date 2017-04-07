/// Interleave the two vectors (and emptying them).
fn interleave(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    interleave_helper(v1, v2, &mut result);
    result
}

fn interleave_helper(v1: &mut Vec<i32>, v2: &mut Vec<i32>, result: &mut Vec<i32>) -> () {
    if v1.len() != 0 {
        result.push(v1.remove(0));
        interleave_helper(v2, v1, result);
    } else if v2.len() != 0 {
        result.push(v2.remove(0));
        interleave_helper(v2, v1, result);
    }
}

pub fn main() {
    let mut v1: Vec<i32> = vec![1, 23, 13, 4, 6, -2, 18];
    let mut v2: Vec<i32> = vec![5, 6, 14];
    let mut result = interleave(&mut v1, &mut v2);
    println!("{:?}", result);

    let mut v3: Vec<i32> = vec![5, 10];
    let mut v4: Vec<i32> = vec![-25, 4];
    result = interleave(&mut v3, &mut v4);
    println!("{:?}", result);

    let mut v5: Vec<i32> = vec![16, -23, 0];
    let mut v6: Vec<i32> = vec![10, 20, 30, 40, 50];
    result = interleave(&mut v5, &mut v6);
    println!("{:?}", result);
}
