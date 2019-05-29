use std::io;

fn main(){
    println!("Enter Text:");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Some error occurred.");

    let vowels = ['a','e','i','o','u','A','E','I','O','U'];

    let mut vowels_count:u32 = 0;
    let mut consonants_count:u32 = 0;
    
    for elem in text.trim().chars(){        
        if vowels.contains(&elem){
            vowels_count += 1;
        }
        else{
            consonants_count += 1;
        }
    }

    println!("Vowels: {}", vowels_count);
    println!("Consonants: {}", consonants_count);
}