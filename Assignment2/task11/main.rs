use std::io;

fn main(){
    println!("I will add up the numbers you will give me.");

    let mut sum:i32 = 0;

    loop{
        println!("Number: ");
        
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Invalid Number");
        let number:i32 = num.trim().parse().unwrap();

        sum += number;

        if number == 0{
            println!("The total is: {}", sum);

            break;
        }

        println!("The total so far is: {}", sum);
    }
}