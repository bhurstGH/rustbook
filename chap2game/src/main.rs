// Rng trait, defines methods that RNGs implement
use rand::Rng;
// enum, variants: Less, Greater, Equal - possible outcomes from comparing two values
use std::cmp::Ordering;
// Necessary because io module is not included in the prelude
use std::io;

// cargo doc --open - builds doc provided by dependencies locally

fn main() {
    println!("Guess the number!");

    // thread_rng RNG that is local to current thread of execution and sseded by OS
    // gen_range called on RNG - is defined by Rng trait
    // lower bound inclusive, upperbound exclusive
    // type inference is defaulting to i32
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // uncomment for testing
    // println!("The secret number is {}", secret_number);

    // infite loop
    loop {
        println!("Please input your guess.");

        // String = growable, UTF-8 encoded bit of text
        // :: indicates that new() is an associated function of String
        // Like a static method, it is implemented on the type, not the instance
        let mut guess = String::new();

        // std::io:stdin would work if we didn't bring in std::io
        // stdin returns instance of std::io::Stdin - type to handle stdin for terminal
        io::stdin()
            // & for reference, access piece of data without needing to use more memory
            // take user input and place in string. Returns generic io::Result
            // Result types are enums - variants: Ok, Err
            .read_line(&mut guess)
            // Result method
            // if Err, crash, return message
            // if Ok, take Ok value  and return - number of bytes in user input
            .expect("Failed to read line");
        // will compile without, but with warning
        // Should write error handling but expect() is ok to crash this program

        // even though guess already exists, Rust allows us to shadow the previous value with a new one
        // Shadowing often used for converting types, rather than creating new variable

        // trim - eliminate white space on origin guess string
        // pressing enter on read_line() adds newline char

        // parse parses string into some kind of number, taken from : u32 type annotation
        // returns Result type, expect() handles Err incase input can't be converted to number
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // match instead of expect for error handling
        // parse returns Result type - Ok with resulting number or Err
        // underscore is catchall value - match all Err values
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} are little grab pincers holding values in place. This is science.
        println!("You guessed: {}", guess);

        // cmp takes a reference for comparison, returns Ordering variant
        // match expression is like switch, executes based on which variant was returned
        // match is made up of arms, which consist of patterns, and code to run if value matches arm pattern
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner! Thanks for playing!");
                break;
            }
        }
    }
}
