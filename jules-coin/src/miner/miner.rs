use crate::blockchain::blockchain::Blockchain;
use crate::wallet::wallet::Wallet;

pub struct Miner {
    pub wallet: Wallet,
}

impl Miner {
    pub fn new(wallet: Wallet) -> Self {
        Miner { wallet }
    }

    pub fn mine(&self, blockchain: &mut Blockchain) {
        blockchain.mine_pending_transactions(self.wallet.public_key.clone());
    }
}
