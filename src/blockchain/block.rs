use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::transaction::Transaction;

#[derive(Hash)]
pub struct Block {
    pub id: u64,
    pub prev_hash: Option<u64>,
    transaction_list: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, prev_hash: impl Into<Option<u64>>, transactions: Vec<Transaction>) -> Block {
        let mut transaction_list = Vec::new();
        for transaction in transactions {
            transaction_list.push(transaction)
        }
        Block {
            id: id,
            prev_hash: prev_hash.into(),
            transaction_list: transaction_list,
            nonce: 0,
        }
    }

    pub fn print(&self) {
        let prev_hash: u64;
        match self.prev_hash {
            Some(hash) => prev_hash = hash,
            None => prev_hash = u64::MAX,
        }

        println!("----Block {}----\nprev_hash: {:#x}\nhash: {:#x}\nnonce: {}", self.id, prev_hash, self.get_hash(), self.nonce);
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}