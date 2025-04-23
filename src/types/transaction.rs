use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct Transaction {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub timestamp: u128,
    pub program_id: String,
}
