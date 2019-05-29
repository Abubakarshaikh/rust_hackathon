extern crate digits_iterator;
use std::io;
use digits_iterator::*;

fn main(){
    println!("Enter a number.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter a valid number.");

    let input_int:u64 = input.trim().parse::<u64>().unwrap();

    let digits:Vec<_> = input_int.digits().collect();

    let mut result = 0;
    let mut text = String::from("The sum of ");    

    for elem in digits.iter(){
        result+=elem;
        text.push_str(&elem.to_string());
        text.push_str("+");
    }
    
    text.pop();
    text.push_str(" is: ");
    text.push_str(&result.to_string());

    println!("{}", text);
}