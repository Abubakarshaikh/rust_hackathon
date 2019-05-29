use std::io;

fn main(){
    println!("Input Radius:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Enter a valid number");
    
    let input_int:f64 = match input.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Error here.")
    };
    
    let area = calculate_area(input_int);
    println!("The area of a circle is: {}", area);
}

fn calculate_area(value:f64) -> f64{
    const PI:f64 = 3.1419;

    PI * (value * value)
}