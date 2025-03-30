use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

fn main() {
    // Connect to the Solana devnet
    let devnet_url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(devnet_url.to_string());
    println!("⚡️ Connected to devnet");

    // Define the public key
    let public_key_str = "9Uhv7PuBYAGrcHUzF4zCbDAFT4J78nUKfsy632gVvTpq";
    let public_key = Pubkey::from_str(public_key_str).expect("Invalid public key");

    // Check balance and airdrop if necessary
    let balance_in_lamports = rpc_client
        .get_balance(&public_key)
        .expect("Failed to get balance");
    let min_balance = (0.5 * LAMPORTS_PER_SOL as f64) as u64;
    if balance_in_lamports < min_balance {
        let airdrop_amount = LAMPORTS_PER_SOL; // 1 SOL
        let signature = rpc_client
            .request_airdrop(&public_key, airdrop_amount)
            .expect("Airdrop failed");
        rpc_client
            .confirm_transaction(&signature)
            .expect("Airdrop not confirmed");
        println!("Airdropped 1 SOL to {}", public_key);
    }

    // Get the updated balance
    let balance_in_lamports = rpc_client
        .get_balance(&public_key)
        .expect("Failed to get balance");
    let balance_in_sol = balance_in_lamports as f64 / LAMPORTS_PER_SOL as f64;
    println!(
        "💰 The balance for the wallet at address {} is: {}",
        public_key, balance_in_sol
    );
}
