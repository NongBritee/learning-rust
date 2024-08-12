fn main() {
    let x = 5;
    {
        let y = 9;
        println!("{}, {}", x, y);
    }

    // println!("{}, {}", x, y); Error !
}
