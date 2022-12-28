use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Random num
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");


    println!("Please input your guess.");

    loop {
        // let variable assigned once, immutable
        // let mut, variable can be changed later
        // ::new() is function implemented on String type

        let mut guess = String::new();

        // &mut <var> is mutable reference to mutable variable
        // &<var> is unmutable reference
        //
        // guess is borrowed by read_line?
        // expect unwraps Result prints error on Error
        // if Ok, continue
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Result is enum with multiple Variants
        // Ok and Err

        // into unsigned 32 bit integer
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // inline variable printing
        // ownership?
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
