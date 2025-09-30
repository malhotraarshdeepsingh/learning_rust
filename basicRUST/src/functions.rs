fn main() {
    let sentence: String = String::from("Hello, World!");
    let first_word = get_first_word(sentence);
    println!("First word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::new();
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}