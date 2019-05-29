use std::io;

fn main(){
    loop{
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).expect("Invalid Input.");
        
        let v: Vec<&str> = expression.split(|c: char| 
            "+".contains(c) || "-".contains(c) || "*".contains(c) || "/".contains(c) || "^".contains(c))
            .collect();

        let mut result = 0;
        let number1:i32 = match v[0].trim().parse(){
            Ok(result) => result,
            Err(_) => panic!("You have provided invalid input. A number is expected.")
        };

        if number1 == 0{
            println!("Bye, now.");
            break;
        }

        let number2:i32 = match v[1].trim().parse(){
            Ok(result) => result,
            Err(_) => panic!("You have provided invalid input. A number is expected.")
        };
        
        if expression.contains("+"){
                result = number1 + number2;
            }
            else if expression.contains("-"){
                result = number1 - number2;
            }
            else if expression.contains("*"){
                result = number1 * number2;
            }
            else if expression.contains("/"){
                result = number1 / number2;
            }
            else if expression.contains("^"){
                result = calculate_power(number1, number2);
            }
            else{
                println!("Invalid operation, bye bye.");
                break;
            }

        println!("{}", result);
    }
}

fn calculate_power(num1:i32, num2:i32) -> i32{
    let mut result = 1;

    for i in 1..num2+1{
        result = num1 * result;
    }

    result
}