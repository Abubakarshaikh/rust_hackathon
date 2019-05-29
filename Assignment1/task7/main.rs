use std::io;

fn main(){
    println!("Enter a character:");
    let mut letter = String::new();
    io::stdin().read_line(&mut letter).expect("Enter a valid character here.");

    let letter_char:char = match letter.trim().parse(){
        Ok(result) => result,
        Err(_) => panic!("panic here.")
    };

    let vowel_array = ['A','E','I','O','U','a','e','i','o','u'];

    if vowel_array.contains(&letter_char){
        println!("Letter {} is vowel.", letter.trim());
    }
    else {
        println!("Letter {} is not a vowel.", letter.trim());
    }     
}