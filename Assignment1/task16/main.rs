use std::io;

fn main(){
    println!("Enter a binary number:");
    let mut binary = String::new();
    io::stdin().read_line(&mut binary).expect("Enter a valid number.");
    let binary_int:String = binary.trim().parse::<String>().unwrap();

    let decimal = isize::from_str_radix(&binary_int, 2).unwrap();

    println!("Decimal representation of {} is {}", binary, decimal);
}