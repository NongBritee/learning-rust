const STARTING_MISSILE: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // immutable by default, can't change value
    // let bunnies = 2;
    // bunnies = 3; error

    // mutable variable
    // let mut rabbits = 3;

    // const WARP_FACTOR: f64 = 9.9;
    // const must be upper & snake case, type must be annotated and assigned
    // immutable, global

    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILE, READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
