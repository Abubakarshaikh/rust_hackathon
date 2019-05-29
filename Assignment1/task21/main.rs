fn main(){
    for i in (1..6){
        for j in (1..i){
            print!("{}", j);
        }

        println!("{}", i);
    }

    for i in (1..5).rev(){
        for j in (1..i){
            print!("{}", j);
        }

        println!("{}", i);
    }
}