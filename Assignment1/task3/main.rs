use std::io;

fn main(){
    println!("Enter Numerator:");
    let mut numerator = String::new();
    io::stdin().read_line(&mut numerator).expect("Enter a valid number");

    println!("Enter Denominator:");
    let mut denominator = String::new();
    io::stdin().read_line(&mut denominator).expect("Enter a valid number");

    let num_int:u32 = match numerator.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Panic here.")
    };

    let denom_int:u32 = match denominator.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("Panic here.")
    };

    if num_int % denom_int == 0{
        println!("Number {} is completely divisible by {}", num_int, denom_int);
    }
    else{
        println!("Number {} is not completely divisible by {}", num_int, denom_int);
    }

}