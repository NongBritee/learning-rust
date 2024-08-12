use module_system::greet;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    // module_system::greet();

    // use module_system:greet; to use greet function from module_system
    greet();

    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen_range(0.0..1.0);
    println!("Random number: {}", x);
}
