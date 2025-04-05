use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::process;

fn main() {
    // Load environment variables from .env file, if present
    dotenvy::dotenv().ok();

    // Retrieve the private key from the "PK" environment variable
    let pk = std::env::var("SECRET_KEY").unwrap_or_else(|_| {
        eprintln!("No private key provided");
        process::exit(1);
    });

    // Parse the private key from a JSON string into a Vec<u8>
    let as_bytes: Vec<u8> = serde_json::from_str(&pk).expect("Failed to parse private key");

    // Ensure the private key is exactly 64 bytes
    if as_bytes.len() != 64 {
        eprintln!("Private key must be 64 bytes");
        process::exit(1);
    }

    // Create a keypair from the byte array
    let keypair = Keypair::from_bytes(&as_bytes).expect("Invalid private key");

    // Print the public key in Base58 format
    println!("Public key: {}", keypair.pubkey().to_string());
}
