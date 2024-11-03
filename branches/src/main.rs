fn main() {
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");

    let a = Box::new([0; 500]);
    for (i, elem) in a.iter().enumerate() {
        println!("Index {:?}: {:?}", i, elem);
    }
  }