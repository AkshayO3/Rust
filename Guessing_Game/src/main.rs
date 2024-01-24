use std::cmp::Ordering;
use std::io;    // Importing the necessary library
use rand::Rng;
use colored::*;

fn main() {
    println!("Hello world");
    let secret:i32 = rand::thread_rng().gen_range(1..=100);    //Generates a random number in the provided range
    println!("Welcome to the guessing game,please enter your input.");

    loop {
        let mut guess = String::new();  //Syntax to enter a new string
        io::stdin().
            read_line(&mut guess).
            expect("Failed to read line.");     //Error handling

        // let guess: i32 = guess.trim().     // Removes any whitespaces around the string
        //     parse().    // The parse function converts,to the data type specified in the left side.
        //     expect("Error in conversion,try entering a valid number.");      // Error handling

        // We need the program to loop until the user enters a valid number,to do this,we use the match function.
        let guess:i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue  //Whatever error,just run the loop again
        };

        println!("The secret number is {}", secret);
        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Equal => {println!("{}","The right guess".green());break;}, // Program quits on the correct guess
            Ordering::Greater => println!("{}","Too high".yellow())
        }
    }
}
