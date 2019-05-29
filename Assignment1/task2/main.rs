use std::io;

fn main(){
    println!("Enter a number");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter a valid number.");

    let input_int:i32 = match input.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Error here.")
    };

    if input_int > 0 {
        println!("Positive number entered.");
    }
    else if input_int < 0 {
        println!("Negative number entered.");
    }
    else {
        println!("Zero entered.");
    }
}