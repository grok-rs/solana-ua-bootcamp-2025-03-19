use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
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

    let recipient = Pubkey::from_str("7BfEoH7SMjX4LJBSF9AJEP3Empnr8vqorRNB3j5c52SG")
        .expect("Invalid recipient public key");
    println!("💸 Attempting to send 0.01 SOL to {}...", recipient);

    let lamports = (0.01 * 1_000_000_000.0) as u64;

    let send_sol_instruction = system_instruction::transfer(&sender.pubkey(), &recipient, lamports);

    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")
        .expect("Invalid Memo program ID");

    let memo_text = "Hello from Solana!";

    let add_memo_instruction = Instruction {
        program_id: memo_program,
        accounts: vec![solana_sdk::instruction::AccountMeta::new(
            sender.pubkey(),
            true,
        )],
        data: memo_text.as_bytes().to_vec(),
    };

    let instructions = vec![send_sol_instruction, add_memo_instruction];

    let recent_blockhash = connection
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let message = Message::new(&instructions, Some(&sender.pubkey()));

    let mut transaction = Transaction::new_unsigned(message);
    transaction.sign(&[&sender], recent_blockhash);

    let signature = connection
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");

    // Output the results
    println!("✅ Transaction confirmed, signature: {}!", signature);
    println!("📝 Memo is: {}", memo_text);
}
