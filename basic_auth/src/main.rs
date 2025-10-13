use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "test_user".to_string();
    let password: Option<String> = None;

    let response = client
        .get("https://httpbin.org/")
        .basic_auth(user, password)
        .send()?;

    if response.status().is_success() {
        println!("Authentication successful!: {:?}", response.text()?);
    } else {
        println!("Authentication failed with status: {}", response.status());
    }

    Ok(())
}