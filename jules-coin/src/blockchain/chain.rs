use super::block::Block;
use super::transaction::Transaction;
use secp256k1::{Secp256k1, Message, PublicKey};
use secp256k1::ecdsa::Signature;
use std::str::FromStr;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, vec![], "0".to_string());
        self.blocks.push(genesis_block);
    }

    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if transaction.sender == "JulesCoin" || self.verify_transaction(&transaction) {
            self.pending_transactions.push(transaction);
        }
    }

    fn verify_transaction(&self, transaction: &Transaction) -> bool {
        if let Some(signature) = &transaction.signature {
            let secp = Secp256k1::new();
            let public_key = PublicKey::from_str(&transaction.sender).unwrap();
            let signature = Signature::from_str(signature).unwrap();
            let message = Message::from_digest_slice(transaction.calculate_hash().as_bytes()).expect("32 bytes");
            secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
        } else {
            false
        }
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_transaction = Transaction::new(
            "JulesCoin".to_string(),
            miner_address,
            1.0,
        );
        self.pending_transactions.push(reward_transaction);

        let previous_hash = self.get_latest_block().hash.clone();
        let mut block = Block::new(
            self.blocks.len() as u64,
            self.pending_transactions.clone(),
            previous_hash,
        );
        block.mine_block(self.difficulty);
        self.blocks.push(block);
        self.pending_transactions = vec![];
    }
}
