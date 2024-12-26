use crate::test_utils::setup_pool_addresses;
use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signer::Signer, system_program},
    Program,
};
use rusted_dex::{accounts, instruction};
use solana_sdk::signature::{Keypair, Signature};
use std::error::Error;

#[test]
pub fn initialize_pool() -> Result<(), Box<dyn Error>> {
    let (pool_addr, token_a, token_b, payer, program) = setup_pool_addresses();

    let tx = initialize_program(pool_addr, token_a, token_b, payer, &program)
        .map_err(|e| format!("❌initialize pool failed: {}", e))?;

    println!("✅initialize pool successful: {}", tx);
    println!("https://explorer.solana.com/tx/{}?cluster=devnet", tx);

    Ok(())
}

pub fn initialize_program(
    pool_addr: Pubkey,
    token_a: Pubkey,
    token_b: Pubkey,
    payer: &'static Keypair,
    program: &Program<&'static Keypair>,
) -> Result<Signature, Box<dyn Error>> {
    let init_tx = program
        .request()
        .accounts(accounts::InitializePool {
            pool: pool_addr,
            user: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(instruction::InitializePool { token_a, token_b })
        .signer(&payer)
        .send()?;

    Ok(init_tx)
}
