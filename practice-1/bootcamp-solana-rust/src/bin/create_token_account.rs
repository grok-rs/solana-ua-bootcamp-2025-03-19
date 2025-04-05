use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
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

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint_account =
        Pubkey::from_str("CSE4HCjQDFoBxtEqAXY3uJaoGiyGsL9Yiu4MXRH1SsAx").unwrap();
    let recipient = Pubkey::from_str("ESGpyX14GdtYT9wGevKyYdNt9ifpTsk8Gkb4hpu7pr94").unwrap();

    let token_account = get_associated_token_address(&recipient, &token_mint_account);

    let account_info = connection.get_account(&token_account).ok();

    if account_info.is_none() {
        // If ATA does not exist, create it
        let create_ata_ix = create_associated_token_account(
            &sender.pubkey(),
            &recipient,
            &token_mint_account,
            &spl_token::id(),
        );

        // Get recent blockhash
        let recent_blockhash = connection.get_latest_blockhash().unwrap();

        // Create and sign transaction
        let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&sender.pubkey()),
            &[&sender],
            recent_blockhash,
        );

        // Send and confirm transaction
        connection
            .send_and_confirm_transaction(&transaction)
            .unwrap();
    }

    // Print the token account address
    println!("Token Account: {}", token_account);

    // Construct explorer link
    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_account
    );

    // Print the result
    println!("✅ Created token account: {}", link);
}
