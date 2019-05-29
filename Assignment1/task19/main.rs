use std::io;

fn main(){
    println!("Enter a text.");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Error.");

    let mut alphabets:u32 = 0;
    let mut number:u32 = 0;
    let mut spaces:u32 = 0;
    let mut special:u32 = 0;

    for elem in text.trim().as_bytes().iter(){
        if (elem >= &65 && elem <= &90) || (elem >= &97 && elem <= &122) {
            alphabets += 1;
        }
        else if elem >= &48 && elem <= &57 {
            number += 1;
        }
        else if elem == &32 {
            spaces += 1;
        } 
        else{
            special += 1;
        }
    }

    println!("Numbers: {}", number);
    println!("Alphabets: {}", alphabets);
    println!("Special Characters: {}", special);
    println!("Spaces: {}", spaces);
    
}