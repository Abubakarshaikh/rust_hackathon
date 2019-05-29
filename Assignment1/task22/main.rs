fn main(){
    for i in (1..10){
        for j in (1..i){
            print!("{}", i);
        }

        println!("{}", i);
    }
}