fn main() {
    let name: &str;
    // println!("{}", name);
    // Error `name` used here but it isn't initialized

    if true {
        name = "Alice";
    } else {
        name = "Bob";
    }
    println!("{}", name);
    // compiler knows that `name` is initialized
}
