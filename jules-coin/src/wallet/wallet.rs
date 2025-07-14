use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;

pub struct Wallet {
    secp: Secp256k1<secp256k1::All>,
    secret_key: SecretKey,
    pub public_key: PublicKey,
}

use crate::blockchain::transaction::Transaction;
use secp256k1::Message;

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        Wallet {
            secp,
            secret_key,
            public_key,
        }
    }

    pub fn get_public_key_string(&self) -> String {
        self.public_key.to_string()
    }

    pub fn sign_transaction(&self, transaction: &mut Transaction) {
        let hash = transaction.calculate_hash();
        let message = Message::from_digest_slice(hash.as_bytes()).expect("32 bytes");
        let sig = self.secp.sign_ecdsa(&message, &self.secret_key);
        transaction.signature = Some(sig.to_string());
    }
}
