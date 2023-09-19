fn main() {
    let mut array = [0, 1, 2, 3, 4];

    let ref_0 = &mut array[1];
    *ref_0 += 1;

    println!("{:?}", array);
    increment_elements(1,3);
}

fn increment_elements(element_1: usize, element_2: usize) {
    let mut array = [0, 1, 2, 3, 4];

    {
        let ref_1 = &mut array[element_1];
        *ref_1 += 1;
    }
    {
        let ref_2 = &mut array[element_2];


        *ref_2 += 1;
    }
    println!("{:?}", array);
}