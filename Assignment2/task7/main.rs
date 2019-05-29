use rand::{thread_rng, Rng};

fn main(){
    let mut rng = thread_rng();

    let mut vector = Vec::new();

    loop{
        let value:u64 = rng.gen_range(0, 100);
        
        if value > 91{
            break;
        }
        
        vector.push(value);
    }

    vector.sort();
    println!("Sorted Vector: {:?}", vector);
    
}