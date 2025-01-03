mod functions;

use crate::functions::{ add_five, subtract_10 };

// everything in rust is immutable by default
fn main() {
    add_five(5);
    subtract_10(16);

    let mut x: i32 = 33;
    println!("{x}");
    x = 50;
    println!("{x}");
}
