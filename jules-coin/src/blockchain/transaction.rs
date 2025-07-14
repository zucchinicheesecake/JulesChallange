use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64) -> Self {
        Transaction { from, to, amount }
    }
}
