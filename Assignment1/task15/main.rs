use std::io;

fn main(){
    println!("Enter a decimal number");
    let mut dec = String::new();
    io::stdin().read_line(&mut dec).expect("Enter a valid decimal number");

    let dec_int:u32 = dec.trim().parse::<u32>().unwrap();

    let binary = format!("{:b}", dec_int);

    println!("Binary representation of {} is {}", dec_int, binary);
}