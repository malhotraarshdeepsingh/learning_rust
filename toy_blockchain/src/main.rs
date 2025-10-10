#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();
    let mut reward = String::new();

    println!("Enter miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);

    println!("Enter difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let difficulty: u32 = difficulty
        .trim()
        .parse::<u32>()
        .expect("were expecting a number");
    println!("Generating genisis block...");

    let mut chain = blockchain::Chain::new(
        miner_address.trim().to_string(),
        reward.trim().parse::<f64>().unwrap_or(100.0),
        difficulty,
    );

    loop {
        println!("Menu:");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change difficulty (current: {})", chain.difficulty);
        println!("4. Change reward (current: {})", chain.reward);
        println!("5. Exit");
        print!("Enter choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                print!("Enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);

                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added!"),
                    false => println!("Transaction failed!"),
                }
            }
            2 => {
                println!("Mining block...");

                let new_block = chain.generate_new_block();
                match new_block {
                    true => println!("Block mined!"),
                    false => println!("Block mining failed!"),
                }
            }
            3 => {
                let mut new_difficulty = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_difficulty);

                let res = chain.update_difficulty(
                    new_difficulty
                        .trim()
                        .parse::<u32>()
                        .unwrap_or(chain.difficulty),
                );
                match res {
                    true => println!("Difficulty updated!"),
                    false => println!("Difficulty update failed!"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);

                let res =
                    chain.update_reward(new_reward.trim().parse::<f64>().unwrap_or(chain.reward));
                match res {
                    true => println!("Reward updated!"),
                    false => println!("Reward update failed!"),
                }
            }
            5 => {
                println!("Exiting...");
                process::exit(0);
            }
            _ => {
                println!("Invalid choice!");
            }
        }
    }
}
