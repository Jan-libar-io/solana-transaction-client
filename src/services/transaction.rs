use solana_sdk::signer::{self, Signer};

use crate::types::Transaction;


pub fn create_transaction() -> Transaction {
    let keypair = signer::keypair::read_keypair_file("/home/dev/.solana/testkey.json").unwrap();
    let keypair2 = signer::keypair::read_keypair_file("/home/dev/.solana/mykey_1.json").unwrap();


    let now = std::time::SystemTime::now();
    let timestamp = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();

    Transaction { sender:keypair.pubkey(), receiver: keypair2.pubkey(), amount: 100, timestamp, program_id: "12345678".to_string() }
}
