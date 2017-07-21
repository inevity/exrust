extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;//Knowing which traits to use and which functions and methods to call from a crate 

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);//gen_range from Rng trait

//    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");



    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        //cmp belong string.
        //Ordering enum type
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
