use std::io;

fn main(){
    println!("Enter String:");
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str);

    println!("How many copies of string you need?:");
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Enter a valid number");

    let count_int:u32 = match count.trim().parse(){
        Ok(i) => i,
        Err(_) => panic!("panic here.")
    };

    let mut result = String::new();
    
    let mut counter:u32 = 0;

    loop {        
        result.push_str(&input_str.trim());
        counter += 1;

        if counter == count_int{
            break;
        }
    }

    println!("{} copies of {} are: {}", count_int, input_str, result);
}