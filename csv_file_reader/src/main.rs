use std::error::Error;

use csv;

fn main() {
    if let Err(err) = read_csv("./username.csv") {
        eprintln!("{}", err);
    }
}

fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
