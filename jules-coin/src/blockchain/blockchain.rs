use super::block::Block;
use super::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
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
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let block = Block::new(
            self.blocks.len() as u64,
            self.pending_transactions.clone(),
            self.get_latest_block().hash.clone(),
        );

        let mined_block = self.mine_block(block);
        self.blocks.push(mined_block);

        self.pending_transactions = vec![];
        self.add_transaction(Transaction::new(
            "JulesCoin".to_string(),
            miner_address,
            1,
        ));
    }

    fn mine_block(&self, mut block: Block) -> Block {
        // In a real implementation, the quote would be provided by the miner.
        let quote = "Jules says: To be, or not to be, that is the question.";
        block.transactions.push(Transaction::new(
            "JulesQuote".to_string(),
            quote.to_string(),
            0,
        ));

        while &block.hash[0..self.difficulty] != "0".repeat(self.difficulty) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
        block
    }
}
