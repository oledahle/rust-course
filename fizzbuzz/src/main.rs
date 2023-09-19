fn main() {
    let _a = (0, 1, 1);
    for n in 0..16 {
        let n_is_three = n % 3 == 0;
        let n_is_five = n % 5 == 0;
        if n_is_three && n_is_five {
            println!("{n}: Fizzbuzz!");
        } else if n_is_three {
            println!("{n}: Fizz");
        } else if n_is_five {
            println!("{n}: Buzz");
        } else {
            println!("{n}");
        }
    }
}
