mod functions;

use functions::add::{add_10, add_five};
use functions::minus::subtract_10;

// everything in rust is immutable by default
fn main() {
    add_five(5);
    add_10(70);
    subtract_10(16);

    let mut x: i32 = 33;
    println!("{x}");
    x = 50;
    println!("{x}");
}
