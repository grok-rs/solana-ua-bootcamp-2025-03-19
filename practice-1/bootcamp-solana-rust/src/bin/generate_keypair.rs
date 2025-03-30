use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

fn main() {
    let keypair = Keypair::new();

    let public_key = keypair.pubkey().to_string();
    let secret_key = keypair.to_bytes();

    println!("The public key is: {}", public_key);
    println!("The secret key is: {:?}", secret_key);
    println!("✅ Finished!");
}
