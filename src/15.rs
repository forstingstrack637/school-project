fn main() {
    let mut rng = rand::thread_rng();
    println!("The random number is: {}", rng.gen_range(1..101));
}
