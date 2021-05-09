pub mod block;
pub mod transaction;

use block::Block;
use transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_block(&mut self) {
        let mut temp_block: Block;
        match self.chain.last() {
            Some(block) => temp_block = block::Block::new(self.chain.len() as u64, block.get_hash(), self.pending_transactions.split_off(self.pending_transactions.len() - 1)),
            None => temp_block = block::Block::new(self.chain.len() as u64, None, self.pending_transactions.split_off(self.pending_transactions.len() - 1)),
        }
        
        'retry:
        for i in 0..std::u64::MAX {
            temp_block.nonce = i;

            if temp_block.id != self.chain.len() as u64 {
                temp_block.id = self.chain.len() as u64;
                temp_block.prev_hash = Some(self.chain.last().expect("Error chaing prev_hash").get_hash());
                continue 'retry
            }
            if (temp_block.get_hash() as u64 & 0x0000_0000_00FF_FFFF_u64) == 0x0u64 {
                // should mutex lock the chain
                self.chain.push(temp_block);
                break;
            }
        }
    }
}