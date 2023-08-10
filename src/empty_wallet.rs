use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    pubkey::Pubkey,
    system_instruction::transfer,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

pub fn empty_wallet() -> Result<(), Box<dyn std::error::Error>> {
    let keypair = read_keypair_file("dev-wallet.json")?;

    let rpc_client = RpcClient::new(RPC_URL);

    let to_pubkey = Pubkey::from_str("FoJUCuKaHFE1J3k5GSvBd6DLUfavXRUzAq6bYt5LVBq7")?;

    let balance = rpc_client.get_balance(&keypair.pubkey())?;

    println!("Balance Is: {}", balance);

    let recent_blockhash = rpc_client.get_latest_blockhash()?;

    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

    println!("Fee Required is: {}", fee);

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );

    let signature = rpc_client.send_transaction(&transaction)?;
    println!("Transaction Signature: {:?}", signature);

    Ok(())
}
