use std::io;

fn main(){
    println!("Enter a coordinate for x1:");
    let mut x1 = String::new();
    io::stdin().read_line(&mut x1).expect("Enter a valid number.");
    let x1_int:f64 = match x1.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    println!("Enter a coordinate for x2:");
    let mut x2 = String::new();
    io::stdin().read_line(&mut x2).expect("Enter a valid number.");
    let x2_int:f64 = match x2.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    println!("Enter a coordinate for y1:");
    let mut y1 = String::new();
    io::stdin().read_line(&mut y1).expect("Enter a valid number.");
    let y1_int:f64 = match y1.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    println!("Enter a coordinate for y2:");
    let mut y2 = String::new();
    io::stdin().read_line(&mut y2).expect("Enter a valid number.");
    let y2_int:f64 = match y2.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    let p1:f64 = (y1_int - x1_int) * (y1_int - x1_int);
    let q1:f64 = (y2_int - x2_int) * (y2_int - x2_int);

    let result:f64 = p1 + q1;
    println!("Distance between points ({},{}) and ({},{}) is {}", x1_int, y1_int, x2_int, y2_int, result.sqrt());
}