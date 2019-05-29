use std::io;

fn main(){
    println!("Enter height in feet:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Enter a valid number here.");

    let height_float:f64 = match height.trim().parse() {
        Ok(expr) => expr,
        Err(_) => panic!("Panic here")
    };

    let result:f64 = height_float * 30.48;

    println!("There are {} Cms in {} foot.", result, height_float);
}