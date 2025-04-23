use std::sync::mpsc::Receiver;
use solana_sdk::transaction::Transaction;

pub fn sequencer(channel_reciever: Receiver<Transaction>) {
    match channel_reciever.recv() {
        Ok(transaction) => {
            println!("Received: {:?}", transaction);
        }
        Err(e) => {
            eprintln!("Error receiving message: {:?}", e);
        }
    }
}
