mod airdrop;

use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use airdrop::airdrop;

fn keygen() {
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

fn main() {
    println!("Calling the airdrop function:");
    airdrop();
}
