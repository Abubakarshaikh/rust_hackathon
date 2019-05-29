use std::io;

struct Triangle{
    base:u32,
    height:u32
}


fn main(){
    println!("Enter magnitude of triangle base:");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Enter a valid number here.");

    println!("Enter magnitude of triangle height:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Enter a valid number here.");

    let base_int:u32 = match base.trim().parse(){
        Ok(input) => input,
        Err(_) => panic!("panic here.")
    };

    let height_int:u32 = match height.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    let triangle = Triangle { base: base_int, height: height_int };    

    println!("Area of a triangle with height {} and base {} is: {}", height_int, base_int, calculate_area(&triangle));
}

fn calculate_area(triangle:&Triangle) -> u32 {
    (triangle.base * triangle.height) / 2
}