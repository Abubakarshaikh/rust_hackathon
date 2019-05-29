use std::io;

fn main(){
    println!("Enter Text:");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Some error occurred.");

    let mut reverse_text = text.to_string();
    reverse_text = reverse_text.trim().to_string();
    
    let mut result = false;

    for elem in text.trim().chars(){
        for prop in reverse_text.trim().chars().rev(){
            if elem != prop {
                println!("The string {} is not a palindrome.", text.trim());
                result = false;
                break;
            }
            else {
                result = true;
                break;
            }
        }

        reverse_text.pop();

        if result == false {
                break;
            }
        
    }
 
 if result  == true{
println!("The string {} is a palindrome.", text.trim());
 }
    
}