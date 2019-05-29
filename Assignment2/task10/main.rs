use rand::{thread_rng, Rng};
use std::io;
use std::cmp::Ordering;

fn main(){
    println!("I'm thinking of a number between 1 - 100. You have 7 guesses.");
    
    let mut rng = thread_rng();
    let guesses_number:u32 = rng.gen_range(1, 101);
        
    let mut counter = 0;
    loop{
        counter += 1;
    
        let mut input_number = 0;
    
        if counter == 1{
            let mut input = String::new();
            println!("First Guess:");
            io::stdin().read_line(&mut input).expect("Invalid number.");

            input_number = input.trim().parse::<u32>().unwrap();
        }
        else{
            let mut input = String::new();
            println!("Guess # {}:", counter);
            io::stdin().read_line(&mut input).expect("Invalid number.");

            input_number = input.trim().parse::<u32>().unwrap();
        }

        match input_number.cmp(&guesses_number){
            Ordering::Greater => println!("Sorry, you are too high."),
            Ordering::Less => println!("Sorry, you are too low."),
            Ordering::Equal => {
                println!("You guessed it! What are the odds?!?");
                
                break;
            }
        }
        
        if counter == 7{
            println!("Sorry, you didn't guess in 7 tries. You lose.");

            break;
        }
    }
}