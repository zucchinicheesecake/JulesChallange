use super::transaction::Transaction;
use chrono::prelude::*;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut headers = self.index.to_string();
        headers.push_str(&self.timestamp.to_string());
        headers.push_str(&serde_json::to_string(&self.transactions).unwrap());
        headers.push_str(&self.previous_hash);
        headers.push_str(&self.nonce.to_string());

        let mut hasher = Sha256::new();
        hasher.update(headers);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while &self.hash[..difficulty] != prefix {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}
