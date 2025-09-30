fn main() {
    let mut users: HashMap<String, i32> = HashMap::new();

    users.insert(String::from("John Doe"), 123);
    users.insert(String::from("Jane Doe"), 456);

    let first_user_age = users.get("John Doe");

    match first_user_age {
        Some(age) => {
            println!("Age of John Doe: {}", age);
        }
        None => {
            println!("No age found");
        }
    }
}