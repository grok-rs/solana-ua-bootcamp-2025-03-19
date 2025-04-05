use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    program_pack::Pack,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_token::{instruction::initialize_mint, state::Mint};
use std::env;

fn main() {
    dotenv().ok();

    let private_key_str = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let private_key_bytes: Vec<u8> =
        serde_json::from_str(&private_key_str).expect("Invalid JSON format for private key");
    let sender = Keypair::from_bytes(&private_key_bytes).expect("Invalid keypair bytes");

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint = Keypair::new();

    let space = Mint::LEN;
    let rent = connection
        .get_minimum_balance_for_rent_exemption(space)
        .expect("Failed to get rent exemption");

    let create_account_ix = system_instruction::create_account(
        &sender.pubkey(),
        &token_mint.pubkey(),
        rent,
        space as u64,
        &spl_token::id(),
    );

    let initialize_mint_ix = initialize_mint(
        &spl_token::id(),
        &token_mint.pubkey(),
        &sender.pubkey(),
        None, // No freeze authority
        2,    // 2 decimal places
    )
    .expect("Failed to create initialize mint instruction");

    let recent_blockhash = connection
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&sender.pubkey()),
        &[&sender, &token_mint],
        recent_blockhash,
    );

    let _signature = connection
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_mint.pubkey()
    );

    println!("✅ Token Mint: {}", link);
}
