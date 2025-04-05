use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::instruction::mint_to;
use std::env;
use std::str::FromStr;

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

    const MINOR_UNITS_PER_MAJOR_UNITS: u64 = 100; // 10^2

    let token_mint_account =
        Pubkey::from_str("CSE4HCjQDFoBxtEqAXY3uJaoGiyGsL9Yiu4MXRH1SsAx").unwrap();
    let recipient_associated_token_account =
        Pubkey::from_str("Gnd6d7BFB5489ig5a956fKrBGa6kR5cajs2HAYJJjh4Q").unwrap();

    let mint_to_ix = mint_to(
        &spl_token::id(),
        &token_mint_account,
        &recipient_associated_token_account,
        &sender.pubkey(),
        &[],
        10 * MINOR_UNITS_PER_MAJOR_UNITS,
    )
    .unwrap();

    let recent_blockhash = connection.get_latest_blockhash().unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let signature = connection
        .send_and_confirm_transaction(&transaction)
        .unwrap();

    let link = format!(
        "https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );

    println!("✅ Success!");
    println!("Mint Token Transaction: {}", link);
}
