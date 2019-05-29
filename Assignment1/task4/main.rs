use std::io;

fn main(){
    println!("Enter Radius of sphere:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter a valid number");

    let input_int: f64 = match input.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Panic here")
    };
    
    println!("Volume of a sphere with radius {} is {}", input_int, calculate_volume(input_int));
}

fn calculate_volume(value:f64) -> f64 {
    const  PI:f64 = 3.14159;

    (4.0/3.0) * PI * (value * value * value)
}