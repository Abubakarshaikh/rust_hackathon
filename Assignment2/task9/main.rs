use rand::{thread_rng, Rng};

fn main(){
    let mut rng = thread_rng();

    let dice1:u32 = rng.gen_range(1, 6);
    let dice2:u32 = rng.gen_range(1, 6);

    println!("HERE COMES THE DICE!");
    println!("Roll # 1: {}", dice1);
    println!("Roll # 1: {}", dice2);
    println!("The Total is: {}", dice1 + dice2);
}