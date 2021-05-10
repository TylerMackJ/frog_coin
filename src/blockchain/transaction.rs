use std::hash::Hash;

#[derive(Hash, Copy, Clone)]
pub struct Transaction {
    sender: u64,
    receiver: u64,
    amount: u64,
}

impl Transaction {
    pub fn new(sender: u64, receiver: u64, amount: u64) -> Transaction {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }
}