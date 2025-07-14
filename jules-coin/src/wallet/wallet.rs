use crate::blockchain::transaction::Transaction;
use crate::crypto::hasher::hash_to_string;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Self {
        let private_key = "my_private_key"; // In a real implementation, this would be randomly generated
        let public_key = hash_to_string(private_key.as_bytes());
        Wallet {
            private_key: private_key.to_string(),
            public_key,
        }
    }

    pub fn create_transaction(&self, to: String, amount: u64) -> Transaction {
        Transaction::new(self.public_key.clone(), to, amount)
    }
}
