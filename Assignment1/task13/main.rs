use std::io;

fn main(){
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter a valid number");

    let input_int:u32 = match input.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    let mut result:u32 = 0;
    let mut counter:u32 = 0;
    
    while input_int > counter{
        counter += 1;
        result = result + counter;        
    }

    println!("Sum of n positive integers till {} is {}", input_int, result);
}