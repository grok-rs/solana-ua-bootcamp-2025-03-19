use dotenvy::dotenv;
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
};
use mpl_token_metadata::types::DataV2;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program, sysvar,
    transaction::Transaction,
};
use std::env;
use std::str::FromStr;

fn main() {
    dotenv().ok();

    let private_key_str = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let private_key_bytes: Vec<u8> =
        serde_json::from_str(&private_key_str).expect("Invalid JSON format for private key");
    let user = Keypair::from_bytes(&private_key_bytes).expect("Invalid keypair bytes");

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let token_metadata_program_id =
        Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
    let token_mint_account =
        Pubkey::from_str("CSE4HCjQDFoBxtEqAXY3uJaoGiyGsL9Yiu4MXRH1SsAx").unwrap();

    let metadata_data = DataV2 {
        name: "Solana UA Bootcamp".to_string(),
        symbol: "UAB-3".to_string(),
        uri: "https://arweave.net/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let (metadata_pda, _metadata_bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            token_metadata_program_id.as_ref(),
            token_mint_account.as_ref(),
        ],
        &token_metadata_program_id,
    );

    let args = CreateMetadataAccountV3InstructionArgs {
        data: metadata_data,
        is_mutable: true,
        collection_details: None,
    };

    let create_metadata = CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint: token_mint_account,
        mint_authority: user.pubkey(),
        payer: user.pubkey(),
        update_authority: (user.pubkey(), true), // Update authority is a signer
        system_program: system_program::id(),
        rent: Some(sysvar::rent::id()), // Rent sysvar is optional but included here
    };

    let instruction = create_metadata.instruction(args);

    let recent_blockhash = connection.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );

    connection
        .send_and_confirm_transaction(&transaction)
        .unwrap();

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_mint_account
    );
    println!("✅ Token mint metadata created: {}", link);
}
