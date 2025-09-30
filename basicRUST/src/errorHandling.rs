use std::{collections::HashMap, fmt::Debug, fs, thread};

fn main() {
    let res = read_from_file_unsafe("hello.txt".to_string());
    println!("Content: {}", res);

    Does not compile
    println!("Hello, world!");
}


fn read_from_file_unsafe(file_content: String) -> String {
    let res = fs::read_to_string(file_content);
    return res.unwrap();
}