// Generate a random integer between 1 and 20
fn main() {
    let x = (rand::random::<i32>() * 20) + 1;
    println!("The random number is {}", x);
}
