use std::io;
use tron_game::*;

fn main() {
    println!("Welcome to Tron!");
    let mut bike = TronBikeState::new(0, 0, Direction::North);
    loop {
        let mut input = String::new();

        println!("Enter one or more a commands ('L', 'R', 'F', or 'quit' to exit):");

        // Read a line of input from the user
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Remove leading and trailing whitespace
        let input = input.trim();

        // Check if the user wants to quit
        if input == "quit" {
            println!("Exiting program.");
            break;
        }

        // Process the user's input (you can replace this with your own logic)
        bike.commands(input);
        bike.print_state();
    }
}







