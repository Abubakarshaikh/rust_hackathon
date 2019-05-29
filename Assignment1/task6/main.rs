use std::io;

fn main(){
    println!("Enter a number: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Enter a valid number here.");

    let number_int:u32 = match number.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Panic here.")
    };

    if number_int % 2 == 0 {
        println!("{} is an even number.", number_int);
    }
    else {
        println!("{} is an odd number.", number_int);
    }
}