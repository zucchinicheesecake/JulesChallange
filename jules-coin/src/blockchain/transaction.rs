use serde::{Deserialize, Serialize};

use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            signature: None,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut headers = self.sender.clone();
        headers.push_str(&self.receiver);
        headers.push_str(&self.amount.to_string());
        let mut hasher = Sha256::new();
        hasher.update(headers);
        format!("{:x}", hasher.finalize())
    }
}
