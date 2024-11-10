/*
Lessons on Structs - The Rust Programming Language
*/
#![allow(unused_variables)]
#![allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count:u64,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}



fn main() {
    let user1 = build_user("jgnoonan@att.net".to_string(), "jgnoonan".to_string());
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("jgnoonan@comcast.net"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{:?}", user2);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Origin: {:?}, Black: {:?}", origin, black);
    let subject = AlwaysEqual;

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    if rect1.width() {
        println!("The rectangle has a non-zero width; it is {} pixels wide.", rect1.width);
    } else {
        println!("The rectangle has a zero width.");
    }
    let sq = Rectangle::square(5);
}
