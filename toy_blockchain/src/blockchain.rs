extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use sha2::{Digest, Sha256};
use std::fmt::Write;

use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockheader {
    timestamp: i64,
    previous_hash: String,
    nonce: u64,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub current_transactions: Vec<Transaction>,
    pub miner_address: String,
    pub reward: f64,
}

impl Chain {
    pub fn new(miner_address: String, reward: f64, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            difficulty,
            current_transactions: Vec::new(),
            miner_address,
            reward,
        };

        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f64) -> bool {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };
        self.current_transactions.push(transaction);
        
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self, reward: f64) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            previous_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_address.clone(),
            amount: self.reward
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![]
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.current_transactions);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println!("New Block Forged: {:#?}", &block);

        self.chain.push(block);
        true
    }

    fn get_merkle(transactions: Vec<Transaction>) -> String {
        let mut merkle: Vec<String> = Vec::new();

        if transactions.is_empty() {
            return String::from_utf8(vec![48; 64]).unwrap();
        }

        for t in &transactions {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 != 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let new_hash = Chain::hash(&h1);
            merkle.push(new_hash);
        }
        merkle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val == 0 {
                        println!("Proof of work found: {}", hash);
                        break;
                    } else {
                        header.nonce += 1;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(item).unwrap();
        let mut hasher = Sha256::default();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let vec_result = result.to_vec();

        Chain::hex_to_string(vec_result.as_slice())
    }

    pub fn hex_to_string(bytes: &[u8]) -> String {
        let mut s = String::new();
        for b in bytes {
            write!(&mut s, "{:x}", b).expect("Unable to write");
        }
        s
    }
}