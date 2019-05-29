use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn main(){
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(30)
            .collect();
    println!("{}", chars);
} 
