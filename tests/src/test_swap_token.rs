use anchor_client::solana_sdk::{
    program_pack::Pack, signature::Keypair, signer::Signer, system_instruction,
};
use anchor_spl::token::{spl_token, Mint};
use rusted_dex::{accounts, instruction, SwapTokenParams};
use std::error::Error;

use crate::test_utils::setup_pool_addresses;

#[test]
fn swap_token() -> Result<(), Box<dyn Error>> {
    let (pool_addr, _token_a, _token_b, payer, program) = setup_pool_addresses();

    // mint accounts keypairs
    let input_token_mint = Keypair::new();
    let output_token_mint = Keypair::new();

    let user_input_token = Keypair::new();
    let user_output_token = Keypair::new();
    let pool_input_token = Keypair::new();
    let pool_output_token = Keypair::new();

    // rent balances
    let rpc_client = program.rpc();
    let mint_rent = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    let token_acct_rent =
        rpc_client.get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?;

    // create mint accounts
    let create_input_mint = system_instruction::create_account(
        &payer.pubkey(),
        &input_token_mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let create_output_mint = system_instruction::create_account(
        &payer.pubkey(),
        &output_token_mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    // initialize mint accounts
    let init_input_mint = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &input_token_mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    );

    let init_output_mint = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &output_token_mint.pubkey(),
        &payer.pubkey(),
        None,
        6,
    );

    // create token accounts
    let create_user_input_token = system_instruction::create_account(
        &payer.pubkey(),
        &user_input_token.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_user_output_token = system_instruction::create_account(
        &payer.pubkey(),
        &user_output_token.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_pool_input_token = system_instruction::create_account(
        &payer.pubkey(),
        &pool_input_token.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_pool_output_token = system_instruction::create_account(
        &payer.pubkey(),
        &pool_output_token.pubkey(),
        token_acct_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    // initialize token accounts
    let init_user_input_token = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &user_input_token.pubkey(),
        &input_token_mint.pubkey(),
        &payer.pubkey(),
    );

    let init_user_output_token = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &user_output_token.pubkey(),
        &output_token_mint.pubkey(),
        &payer.pubkey(),
    );

    let init_pool_input_token = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &pool_input_token.pubkey(),
        &input_token_mint.pubkey(),
        &payer.pubkey(),
    );

    let init_pool_output_token = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &pool_output_token.pubkey(),
        &output_token_mint.pubkey(),
        &payer.pubkey(),
    );

    // mint initial tokens to accounts
    let mint_user_input = spl_token::instruction::mint_to(
        &spl_token::ID,
        &input_token_mint.pubkey(),
        &user_input_token.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        10_000,
    );

    let mint_user_output = spl_token::instruction::mint_to(
        &spl_token::ID,
        &output_token_mint.pubkey(),
        &user_output_token.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        10_000,
    );

    let mint_pool_input = spl_token::instruction::mint_to(
        &spl_token::ID,
        &input_token_mint.pubkey(),
        &pool_input_token.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        1_000_000,
    );

    let mint_pool_output = spl_token::instruction::mint_to(
        &spl_token::ID,
        &output_token_mint.pubkey(),
        &pool_output_token.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        500_000,
    );

    program
        .request()
        .instruction(create_input_mint)
        .instruction(init_input_mint?)
        .instruction(create_output_mint)
        .instruction(init_output_mint?)
        .signer(&input_token_mint)
        .signer(&output_token_mint)
        .send()?;

    program
        .request()
        .instruction(create_user_input_token)
        .instruction(init_user_input_token?)
        .instruction(create_user_output_token)
        .instruction(init_user_output_token?)
        .signer(&user_input_token)
        .signer(&user_output_token)
        .send()?;

    program
        .request()
        .instruction(create_pool_input_token)
        .instruction(init_pool_input_token?)
        .instruction(create_pool_output_token)
        .instruction(init_pool_output_token?)
        .instruction(mint_user_input?)
        .instruction(mint_user_output?)
        .instruction(mint_pool_input?)
        .instruction(mint_pool_output?)
        .signer(&pool_input_token)
        .signer(&pool_output_token)
        .send()?;

    let amount_in = 1000;
    let minimum_output_amount = 450; // slippage protection
    let swap_params = SwapTokenParams {
        amount_in,
        minimum_output_amount,
    };

    let swap_ix = program
        .request()
        .accounts(accounts::SwapToken {
            pool: pool_addr,
            user: payer.pubkey(),
            user_input_token_account: user_input_token.pubkey(),
            user_output_token_account: user_output_token.pubkey(),
            pool_input_token_account: pool_input_token.pubkey(),
            pool_output_token_account: pool_output_token.pubkey(),
            token_program: spl_token::ID,
        })
        .args(instruction::SwapToken {
            params: swap_params,
        })
        .signer(&payer)
        .send()
        .map_err(|e| format!("❌swap token failed: {}", e))?;

    println!("✅swap token successful: {}", swap_ix);
    println!("https://explorer.solana.com/tx/{}?cluster=devnet", swap_ix);

    Ok(())
}
