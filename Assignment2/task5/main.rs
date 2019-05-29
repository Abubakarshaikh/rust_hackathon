fn main() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|param1, param2| param1.partial_cmp(param2).unwrap());

    println!("{:?}", vec);
}
