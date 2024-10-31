#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

fn f(x: i32) -> i32 { x + 1 }
fn main() {
  println!("{}", f({
    let y = 1;
    y + 1
  }));
}