use crate::blockchain::block::Block;

pub trait Consensus {
    fn validate_block(&self, block: &Block) -> bool;
}

pub struct ProofOfJules;

impl Consensus for ProofOfJules {
    fn validate_block(&self, block: &Block) -> bool {
        // For now, we'll just check if the block's hash starts with "00"
        // and if the block contains a Jules-themed quote.
        // In a real implementation, this would be much more complex.
        let quote = "Jules";
        let has_quote = block.transactions.iter().any(|tx| tx.from == "JulesQuote" && tx.to.contains(quote));
        &block.hash[0..2] == "00" && has_quote
    }
}
