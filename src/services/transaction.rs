use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signer::{self, Signer},
    system_instruction,
    transaction::Transaction,
};


pub fn create_transaction() -> Transaction {
    let keypair = signer::keypair::read_keypair_file("/home/dev/.solana/testkey.json").unwrap();
    let keypair2 = signer::keypair::read_keypair_file("/home/dev/.solana/mykey_1.json").unwrap();
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".into());

    let instruction =
        system_instruction::transfer(&keypair2.pubkey(), &keypair.pubkey(), 1 * LAMPORTS_PER_SOL);
    Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair2.pubkey()),
        &[&keypair2],
        rpc_client.get_latest_blockhash().unwrap(),
    )
}
