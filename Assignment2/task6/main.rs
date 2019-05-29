use rand::{thread_rng, Rng};

fn main(){
    let mut rng = thread_rng();

    let mut vector = Vec::new();

    for i in 1..101{
        let value:f64 = rng.gen_range(0.0, 900.0);
        vector.push(value);
    }

    let mut p1:u32 = 0;
    let mut p2:u32 = 0;
    let mut p3:u32 = 0;

    for &elem in vector.iter(){
        if elem >= 0.0 && elem < 300.0{
            p1 += 1;
        }
        else if elem >= 300.0 && elem < 600.0{
            p2 += 1;
        }
        else{
            p3 += 1;
        }
    }

    println!("0 - 300: {}", p1);
    println!("300 - 600: {}", p2);
    println!("Over 600: {}", p3);
    
}