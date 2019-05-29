use rand::{thread_rng, Rng};

fn main(){
    let mut rng = thread_rng();
    let integer: u32 = rng.gen_range(0, 10);
    let float: f64 = rng.gen_range(0.0, 10.0);
    
    println!("Integer: {}", integer);
    println!("Float: {}", float);

}