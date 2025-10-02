struct User<'a> {
    username: &'a String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

impl Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rectangle: {} x {}", self.width, self.height)
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Fix {
    fn fix(&self) -> String;
}

pub fn notify(u: impl Summary) {
    println!("Summary: {}", u.summarize());
}

pub fn notify_both<T: Summary + Fix> (u: T) {
    println!("Fix: {}", u.fix());
}

fn main() {
    // Structs
    let user1 = User {
        username: String::from("John Doe"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("User 1: {}", user1.username);
    println!("User 1: {}", user1.active);
    println!("User 1: {}", user1.email);
    println!("User 1: {}", user1.sign_in_count);

    // Structs with methods
    let rectangle1 = Rectangle {
        width: 3,
        height: 4,
    };
    println!("Area of rectangle1: {}", rectangle1.area());
    println!("{:?}", rectangle1);
}