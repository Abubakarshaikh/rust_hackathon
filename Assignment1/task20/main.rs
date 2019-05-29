fn main(){
    for i in (1..6){
        for j in (1..i){
            print!("*");
        }

        println!("*");
    }

    for i in (1..5).rev(){
        for j in (1..i){
            print!("*");
        }

        println!("*");
    }
}