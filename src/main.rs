mod blockchain;

fn main() {
    let mut bc = blockchain::Blockchain::new();
    bc.add_transaction(blockchain::transaction::Transaction::new(0, 1, 100));
    bc.mine_block();
    bc.chain.last().expect("").print();
    bc.add_transaction(blockchain::transaction::Transaction::new(0, 1, 200));
    bc.add_transaction(blockchain::transaction::Transaction::new(1, 0, 300));
    bc.mine_block();
    bc.chain.last().expect("").print();
}
