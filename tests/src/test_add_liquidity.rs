use anchor_client::solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, system_instruction,
};
use anchor_spl::token::{spl_token, Mint};
use rusted_dex::{accounts, instruction};
use std::error::Error;

use crate::test_utils::setup_pool_addresses;

#[test]
fn add_liquidity() -> Result<(), Box<dyn Error>> {
    let (pool_addr, _token_a, _token_b, payer, program) = setup_pool_addresses();

    // mint accounts keypairs
    let token_a_mint = Keypair::new();
    let token_b_mint = Keypair::new();
    let lp_token_mint = Keypair::new();

    // token accounts keypairs
    let user_token_a = Keypair::new();
    let user_token_b = Keypair::new();
    let pool_token_a = Keypair::new();
    let pool_token_b = Keypair::new();
    let user_lp_token = Keypair::new();

    // rent balances
    let rpc_client = program.rpc();
    let mint_rent = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    let token_acct_rent =
        rpc_client.get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?;

    // create mint accounts
    let create_mint_a = system_instruction::create_account(
        &payer.pubkey(),
        &token_a_mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let create_mint_b = system_instruction::create_account(
        &payer.pubkey(),
        &token_b_mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let create_mint_lp_token = system_instruction::create_account(
        &payer.pubkey(),
        &lp_token_mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    // initialize mint accounts
    let init_mint_a = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &token_a_mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    );

    let init_mint_b = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &token_b_mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    );

    let init_mint_lp_token = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &lp_token_mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    );

    // create token accounts
    let create_user_token_a = system_instruction::create_account(
        &payer.pubkey(),
        &user_token_a.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_user_token_b = system_instruction::create_account(
        &payer.pubkey(),
        &user_token_b.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_pool_token_a = system_instruction::create_account(
        &payer.pubkey(),
        &pool_token_a.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_pool_token_b = system_instruction::create_account(
        &payer.pubkey(),
        &pool_token_b.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_user_lp_token = system_instruction::create_account(
        &payer.pubkey(),
        &user_lp_token.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    // initialize token accounts
    let init_user_token_a = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &user_token_a.pubkey(),
        &token_a_mint.pubkey(),
        &payer.pubkey(),
    );

    let init_user_token_b = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &user_token_b.pubkey(),
        &token_b_mint.pubkey(),
        &payer.pubkey(),
    );

    let init_pool_token_a = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &pool_token_a.pubkey(),
        &token_a_mint.pubkey(),
        &pool_addr,
    );

    let init_pool_token_b = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &pool_token_b.pubkey(),
        &token_b_mint.pubkey(),
        &pool_addr,
    );

    let init_user_lp_token = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &user_lp_token.pubkey(),
        &lp_token_mint.pubkey(),
        &payer.pubkey(),
    );

    // mint initial tokens to accounts
    let mint_to_user_a = spl_token::instruction::mint_to(
        &spl_token::ID,
        &token_a_mint.pubkey(),
        &user_token_a.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        20_000,
    );

    let mint_to_user_b = spl_token::instruction::mint_to(
        &spl_token::ID,
        &token_b_mint.pubkey(),
        &user_token_b.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        20_000,
    );

    program
        .request()
        .instruction(create_mint_a)
        .instruction(init_mint_a?)
        .instruction(create_mint_b)
        .instruction(init_mint_b?)
        .instruction(create_mint_lp_token)
        .instruction(init_mint_lp_token?)
        .signer(&token_a_mint)
        .signer(&token_b_mint)
        .signer(&lp_token_mint)
        .send()
        .map_err(|e| format!("mint accounts creation failed: {}", e))?;

    program
        .request()
        .instruction(create_user_token_a)
        .instruction(init_user_token_a?)
        .instruction(create_user_token_b)
        .instruction(init_user_token_b?)
        .instruction(create_user_lp_token)
        .instruction(init_user_lp_token?)
        .signer(&user_token_a)
        .signer(&user_token_b)
        .signer(&user_lp_token)
        .send()?;

    program
        .request()
        .instruction(create_pool_token_a)
        .instruction(init_pool_token_a?)
        .instruction(create_pool_token_b)
        .instruction(init_pool_token_b?)
        .instruction(mint_to_user_a?)
        .instruction(mint_to_user_b?)
        .signer(&pool_token_a)
        .signer(&pool_token_b)
        .send()?;

    let add_tx = program
        .request()
        .accounts(accounts::AddLiquidity {
            pool: pool_addr,
            user: payer.pubkey(),
            user_token_a: user_token_a.pubkey(),
            user_token_b: user_token_b.pubkey(),
            pool_token_a: pool_token_a.pubkey(),
            pool_token_b: pool_token_b.pubkey(),
            lp_token_mint: lp_token_mint.pubkey(),
            user_lp_token: user_lp_token.pubkey(),
            token_program: spl_token::ID,
        })
        .args(instruction::AddLiquidity {
            amount_a: 1000,
            amount_b: 1000,
        })
        .signer(&payer)
        .send()
        .map_err(|e| format!("❌add liquidity failed: {}", e))?;

    println!("✅add liquidity successful: {}", add_tx);
    println!("https://explorer.solana.com/tx/{}?cluster=devnet", add_tx);

    Ok(())
}
