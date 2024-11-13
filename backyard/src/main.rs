#![allow(unused_imports)]
pub mod garden;

use garden::vegetables::*;

fn main() {
    let plant = Carrot {};
    println!("{:?}", plant);
}
