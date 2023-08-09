use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, read_keypair_file};
use solana_sdk::signature::Signer;
use bs58;
use std::str;

const RPC_URL: &str = "https://api.devnet.solana.com";

pub fn airdrop() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    let secret_key_base58 = bs58::encode(keypair.to_bytes()).into_string();

    let decoded_bytes = bs58::decode(&secret_key_base58)
        .into_vec()
        .expect("Error decoding base58");

    let keypair = Keypair::from_bytes(&decoded_bytes).expect("Error creating Keypair");

    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(signature) => {
            println!("Success! Airdrop TX signature: {}", signature);
            println!(
                "Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
                signature
            );
        }
        Err(err) => {
            println!("Oops, something went wrong: {}", err);
        }
    };
}
