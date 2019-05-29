use std::io;

struct Interest{
    amount:f64,
    rate:f64,
    years:f64
}

fn main(){
    println!("Enter principal amount:");
    let mut p_amount = String::new();
    io::stdin().read_line(&mut p_amount).expect("Enter valid number here.");

    println!("Enter rate of interest in %:");
    let mut i_rate = String::new();
    io::stdin().read_line(&mut i_rate).expect("Enter valid value here.");
    
    println!("Enter number of years:");
    let mut n_years = String::new();
    io::stdin().read_line(&mut n_years).expect("Enter valid number here.");

    let amount_int:f64 = match p_amount.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    let rate_float:f64 = match i_rate.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    let years_int:f64 = match n_years.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("Panic here.")
    };

    let interest = Interest 
            { 
                amount: amount_int, 
                rate: rate_float, 
                years: years_int 
            };

    println!("After {} years, your principal amount {} over an interest rate of {}% will be {}", years_int, amount_int, rate_float, calculate_interest(&interest));
}

fn calculate_interest(interest: &Interest) -> f64 {
    interest.amount * (1.0 + ((interest.rate / 100.0) * interest.years))
}