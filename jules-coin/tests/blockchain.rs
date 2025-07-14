use jules_coin::blockchain::blockchain::Blockchain;
use jules_coin::blockchain::transaction::Transaction;

#[test]
fn test_add_transaction() {
    let mut blockchain = Blockchain::new(2);
    let transaction = Transaction::new("from".to_string(), "to".to_string(), 100);
    blockchain.add_transaction(transaction);
    assert_eq!(blockchain.pending_transactions.len(), 1);
}

#[test]
fn test_mine_pending_transactions() {
    let mut blockchain = Blockchain::new(2);
    let transaction = Transaction::new("from".to_string(), "to".to_string(), 100);
    blockchain.add_transaction(transaction);
    blockchain.mine_pending_transactions("miner".to_string());
    assert_eq!(blockchain.pending_transactions.len(), 1);
    assert_eq!(blockchain.blocks.len(), 2);
}
