use rand::{thread_rng, Rng};
use std::io;

fn main(){
    let mut rng = thread_rng();

    let number_guessed:u32 = rng.gen_range(1, 10);

    println!("I'm thinking of a number from 1 to 10.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid number");
    let input_str = input.trim().parse::<u32>().unwrap();

    println!("Your Guess: {}", input_str);

    if number_guessed == input_str{
        println!("That's right! My secret number was {}", number_guessed);
    }
    else{
        println!("Sorry, but I was really thinking of {}", number_guessed);
    }
}