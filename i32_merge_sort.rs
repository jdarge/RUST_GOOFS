fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut numbers: Vec<i32> = vec![6,7,8,1,2,5,4,5];
    numbers.push(3);

    println!("{:?}", numbers);

    numbers = merge_sort(&numbers.to_vec());

    println!("{:?}", numbers);
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32>{

    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i+=1;
        } else {
            merged.push(right[j]);
            j+=1;
        }
    }

    while i < left.len() {
        merged.push(left[i]);
        i+=1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j+=1;
    }

    return merged;
}

fn merge_sort(vec: &Vec<i32>) -> Vec<i32>{

    if vec.len() < 2 {return vec.to_vec();}

    let size = vec.len()/2;
        
    let left = merge_sort(&mut vec[0..size].to_vec());
    let right = merge_sort(&mut vec[size..].to_vec());
    let merged = merge(&left, &right);

    return merged;
}
