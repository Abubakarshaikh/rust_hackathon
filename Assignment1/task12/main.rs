use std::io;

struct BMI{
    weight:f64,
    height:f64
}

fn main(){
    println!("Enter height in cms:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Enter a valid number here.");

    println!("Enter weight in kg:");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Enter a valid number here.");

    let height_float:f64 = match height.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("panic here.")
    };

    let weight_float:f64 = match weight.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("panic here.")
    };

    let height_meters:f64 = height_float / 100.0; 

    let bmi = BMI { height: height_meters, weight: weight_float };

    let bmi:f64 = calculate_bmi(&bmi);

    println!("Your BMI is: {}", bmi);
}

fn calculate_bmi(bmi: &BMI) -> f64 {
    bmi.weight / bmi.height.powi(2)
}