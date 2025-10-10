fn main() {
    // Variables
    let x: i32 = 5;
    let y = 10; // Type inference
    let sum = x + y;
    println!("Sum of {} and {} is {}", x, y, sum);

    // numeric types
    let a: u8 = 255; // Unsigned 8-bit integer
    let b: i8 = -128; // Signed 8-bit integer
    let c: f32 = 3.14; // 32-bit floating point

    // so on for other numeric types like u16, i16, u32, i32, u64, i64, f64 etc.

    // Variable overflow
    let mut num: i8 = 124; // Mutable variable
    for i in 0..100 {
        num += 127;
    }
    print!("Number: {}", num);

    // Boolean
    let is_male = false;
    let is_above_18 = true;

    // String
    let greeting = String::from("hello world");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(0);
    println!("{}", char1.unwrap());

    let string: &str = "hello world"; // String slice

    let s = String::from("hello world");
    let slice = find_first_word(&s);
    let literal = "hello world";
    println!("{}", slice);
}
