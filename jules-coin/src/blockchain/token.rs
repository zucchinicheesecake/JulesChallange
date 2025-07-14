use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
}

impl Token {
    pub fn new(name: String, symbol: String, total_supply: u64) -> Self {
        Token {
            name,
            symbol,
            total_supply,
        }
    }
}
