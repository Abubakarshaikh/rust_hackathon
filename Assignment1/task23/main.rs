use std::io;
use chrono::offset::Utc;
use chrono::TimeZone;
use chrono::DateTime;


extern crate chrono;

fn main(){
    println!("Enter first date in (dd/mm/yyyy) format:");
    let mut first_date = String::new();
    io::stdin().read_line(&mut first_date).expect("Error.");

    let time = "00:00:00".to_string();

    let first_date:String = first_date.parse::<String>().unwrap();
    let var1 = first_date.trim().to_string() + &time;
    
    println!("Enter second date in (dd/mm/yyyy) format:");
    let mut second_date = String::new();
    io::stdin().read_line(&mut second_date).expect("Error.");

    let second_date:String = second_date.parse::<String>().unwrap();    
    let var2 = second_date.trim().to_string() + &time;

    let utc = Utc;
    let d1 = utc.datetime_from_str(&var2,"%d/%m/%Y %H:%M:%S").unwrap();
    let d2 = utc.datetime_from_str(&var1,"%d/%m/%Y %H:%M:%S").unwrap();

    let duration = d1.signed_duration_since(d2);
    
    println!("There are {} days in between {} and {}", duration.num_days(), first_date.trim(), second_date.trim());    
}