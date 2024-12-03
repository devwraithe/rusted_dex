use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
};
use rusted_dex::{accounts, instruction};
use std::{error::Error, str::FromStr};

#[test]
fn test_initialize_pool() -> Result<(), Box<dyn Error>> {
    // Retrieve anchor wallet from env variable
    let anchor_wallet =
        std::env::var("ANCHOR_WALLET").expect("Environment variable ANCHOR WALLET is not set");

    // Read the keypair for payer from file
    let payer = read_keypair_file(&anchor_wallet).expect("Failed to read keypair file");

    // Initialize client with explicit commitment
    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());

    // Create program with program id from constants
    let program_id = rusted_dex::ID;
    let program = client
        .program(program_id)
        .expect("Failed to initialize program client");

    // Create new keypair for the order
    let pool = Keypair::new();
    let token_a_str = "6WWRQ5vU17nQaDWeU7QCV4htfWzFwhmoDzADsHdKczVP";
    let token_b_str = "FjrLKrEQ95QonQjbE2yoZnKPXQi1X4YayK6BBurWN73H";
    let token_a = Pubkey::from_str(token_a_str).unwrap();
    let token_b = Pubkey::from_str(token_b_str).unwrap();

    // Build and send transaction
    let tx = program
        .request()
        .accounts(accounts::InitializePool {
            pool: pool.pubkey(),
            user: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(instruction::InitializePool { token_a, token_b })
        .signer(&pool)
        .send()
        .map_err(|e| format!("Transaction failed: {}", e))?;

    println!("Transaction successful. Signature: {}", tx);

    Ok(())
}
