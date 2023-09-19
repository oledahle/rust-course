fn main() {
    let string = String::from("gul banan");
    let substring = substring(&string, 1, 6);
    println!("{:?}",substring);

    let vector: Vec<u32> = vec![1, 2, 3, 4];
    assert_eq!(get_elem_or_default(&vector, 2, 0), 3);
    assert_eq!(get_elem_or_default(&vector, 7, 8), 8);

    let vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // do_slice_vec should return the third to eighth element of a passed vector
    // or just return a slice of the entire vector it is not long enough
    let slice = do_slice_vec(&vec, 2, 8);

    println!("vector slice: {:?}", slice);

    // println! borrows vec, even when we pass it as move
    println!("full vector: {:?}", vec);
}

fn substring(s: &String, start: usize, end: usize) -> &str {
   &s[start..end]
}

fn get_elem_or_default(vec: &Vec<u32>, index: usize, default : u32) -> u32 {
    let op_result = vec.get(index);
    return if op_result.is_some() {
        *op_result.unwrap()
    } else {
        default
    }
}

fn do_slice_vec(vec: &[u32], start: usize, end : usize) -> &[u32] {
    if start > end || end > vec.len() {
        &vec[0..vec.len()]
    } else {
        &vec[start..end]
    }
}