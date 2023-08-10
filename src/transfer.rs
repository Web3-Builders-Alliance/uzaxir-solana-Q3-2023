use solana_client::rpc_client::RpcClient;
use solana_program::{
    pubkey::Pubkey,
    system_instruction::transfer,
};
use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction
};

const RPC_URL: &str = "https://api.devnet.solana.com";



use std::str::FromStr;

pub fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    let secret_key_base58 = bs58::encode(keypair.to_bytes()).into_string();

    let decoded_bytes = bs58::decode(&secret_key_base58)
        .into_vec()
        .expect("Error decoding base58");

    let keypair = Keypair::from_bytes(&decoded_bytes).expect("Error creating Keypair");

    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
    let to_pubkey = Pubkey::from_str("9aHNonohnvk4uTSk1iCunPwDLZ9r3D6A3jTjSi5atnB3").unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(
            &keypair.pubkey(),
            &to_pubkey,
            1_000_000
        )],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash
    );
    let signature = rpc_client
    .send_and_confirm_transaction(&transaction)
    .expect("Failed to send transaction");
    println!(
        "Success! Check out your TX here:
        https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}