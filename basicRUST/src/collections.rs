use std::collections::HashMap;

fn main() {
    // Vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let evens = even_numbers(&v);

    // initialising using rust macro
    let num = vec![1, 2, 3, 4, 5];
    let even = even_numbers(&num);

    println!("{:?}", v);
    println!("{:?}", evens);
    println!("{:?}", num);
    println!("{:?}", even);

    // Hashmaps
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("Alice"), 23);
    users.insert(String::from("Bob"), 34);

    let user1 = users.get("Alice");
    match user1 {
        Some(age) => println!("{}", age),
        None => println!("User not found"),
    }

    let input = vec![
        (String::from("Alice"), 23),
        (String::from("Bob"), 34),
        (String::from("Alice"), 30),
    ];

    let grouped = group_values_by_key(input);
    println!("{:?}", grouped);
}

fn even_numbers(vec: &Vec<i32>) -> Vec<i32> {
    let mut evens = Vec::new();

    for i in vec {
        if i % 2 == 0 {
            evens.push(*i);
        }
    }

    evens
}

fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    for (key, value) in pairs {
        map.insert(key, value);
    }

    map
}
