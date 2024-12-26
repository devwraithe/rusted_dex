// use crate::test_utils::{get_program, initialize_client};
// use anchor_client::solana_sdk::program_pack::Pack;
// use anchor_client::solana_sdk::{
//     signature::Keypair, signer::Signer, system_instruction, system_program,
// };
// use anchor_spl::token::spl_token;
// use rusted_dex::{accounts, instruction};
// use spl_token::state::Mint;
// use std::error::Error;
//
// #[test]
// fn test_add_liquidity() -> Result<(), Box<dyn Error>> {
//     // initialize client and payer
//     let (client, payer) = initialize_client();
//     let program = get_program(&client);
//
//     // // create keypairs from token strings
//     // let pool = Keypair::new();
//     // let token_a_str = "6WWRQ5vU17nQaDWeU7QCV4htfWzFwhmoDzADsHdKczVP";
//     // let token_b_str = "FjrLKrEQ95QonQjbE2yoZnKPXQi1X4YayK6BBurWN73H";
//     // let token_a = Pubkey::from_str(token_a_str).unwrap();
//     // let token_b = Pubkey::from_str(token_b_str).unwrap();
//     //
//     // // built the init pool tx
//     // let init_tx = program
//     //     .request()
//     //     .accounts(accounts::InitializePool {
//     //         pool: pool.pubkey(),
//     //         user: payer.pubkey(),
//     //         system_program: system_program::ID,
//     //     })
//     //     .args(instruction::InitializePool { token_a, token_b })
//     //     .signer(&pool)
//     //     .send()
//     //     .map_err(|e| format!("Transaction failed: {}", e))?;
//     //
//     // println!("Pool initialized with tx: {}", init_tx);
//
//     // create keypairs for all accounts
//     let pool = Keypair::new();
//     let token_a_mint = Keypair::new();
//     let token_b_mint = Keypair::new();
//     let lp_token_mint = Keypair::new();
//
//     // // create token accounts for user
//     // let user_token_a = Keypair::new();
//     // let user_token_b = Keypair::new();
//     // let user_lp_token = Keypair::new();
//     // let pool_token_a = Keypair::new();
//     // let pool_token_b = Keypair::new();
//
//     // mint accounts rent
//     let rent = program
//         .rpc()
//         .get_minimum_balance_for_rent_exemption(Mint::LEN)?;
//
//     // create mint accounts
//     let create_mint_a = system_instruction::create_account(
//         &payer.pubkey(),
//         &token_a_mint.pubkey(),
//         rent,
//         Mint::LEN as u64,
//         &spl_token::ID,
//     );
//
//     let create_mint_b = system_instruction::create_account(
//         &payer.pubkey(),
//         &token_b_mint.pubkey(),
//         rent,
//         Mint::LEN as u64,
//         &spl_token::ID,
//     );
//
//     let create_lp_token = system_instruction::create_account(
//         &payer.pubkey(),
//         &lp_token_mint.pubkey(),
//         rent,
//         Mint::LEN as u64,
//         &spl_token::ID,
//     );
//
//     // initialize the created mint accounts
//     let init_mint_a = spl_token::instruction::initialize_mint(
//         &spl_token::ID,
//         &token_a_mint.pubkey(),
//         &payer.pubkey(),
//         None,
//         6,
//     );
//     let init_mint_b = spl_token::instruction::initialize_mint(
//         &spl_token::ID,
//         &token_b_mint.pubkey(),
//         &payer.pubkey(),
//         None,
//         6,
//     );
//     let init_lp_mint = spl_token::instruction::initialize_mint(
//         &spl_token::ID,
//         &lp_token_mint.pubkey(),
//         &pool.pubkey(), // pool is the mint authority
//         None,
//         6,
//     );
//
//     // // token accounts rent
//     // let token_account_rent = program
//     //     .rpc()
//     //     .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?;
//     //
//     // // create user token accounts
//     // let create_user_token_a = system_instruction::create_account(
//     //     &payer.pubkey(),
//     //     &user_token_a.pubkey(),
//     //     token_account_rent,
//     //     spl_token::state::Account::LEN as u64,
//     //     &spl_token::ID,
//     // );
//     //
//     // let create_user_token_b = system_instruction::create_account(
//     //     &payer.pubkey(),
//     //     &user_token_b.pubkey(),
//     //     token_account_rent,
//     //     spl_token::state::Account::LEN as u64,
//     //     &spl_token::ID,
//     // );
//     //
//     // let create_user_lp_token = system_instruction::create_account(
//     //     &payer.pubkey(),
//     //     &user_lp_token.pubkey(),
//     //     token_account_rent,
//     //     spl_token::state::Account::LEN as u64,
//     //     &spl_token::ID,
//     // );
//     // let create_pool_token_a = system_instruction::create_account(
//     //     &payer.pubkey(),
//     //     &pool_token_a.pubkey(),
//     //     token_account_rent,
//     //     spl_token::state::Account::LEN as u64,
//     //     &spl_token::ID,
//     // );
//     // let create_pool_token_b = system_instruction::create_account(
//     //     &payer.pubkey(),
//     //     &pool_token_b.pubkey(),
//     //     token_account_rent,
//     //     spl_token::state::Account::LEN as u64,
//     //     &spl_token::ID,
//     // );
//     //
//     // // initialize created token accounts
//     // let init_user_token_a = spl_token::instruction::initialize_account(
//     //     &spl_token::ID,
//     //     &user_token_a.pubkey(),
//     //     &token_a_mint.pubkey(),
//     //     &payer.pubkey(),
//     // )?;
//     //
//     // let init_user_token_b = spl_token::instruction::initialize_account(
//     //     &spl_token::ID,
//     //     &user_token_b.pubkey(),
//     //     &token_b_mint.pubkey(),
//     //     &payer.pubkey(),
//     // )?;
//     //
//     // let init_user_lp_token = spl_token::instruction::initialize_account(
//     //     &spl_token::ID,
//     //     &user_lp_token.pubkey(),
//     //     &lp_token_mint.pubkey(),
//     //     &payer.pubkey(),
//     // )?;
//     //
//     // let init_pool_token_a = spl_token::instruction::initialize_account(
//     //     &spl_token::ID,
//     //     &pool_token_a.pubkey(),
//     //     &token_a_mint.pubkey(),
//     //     &pool.pubkey(),
//     // )?;
//     //
//     // let init_pool_token_b = spl_token::instruction::initialize_account(
//     //     &spl_token::ID,
//     //     &pool_token_b.pubkey(),
//     //     &token_b_mint.pubkey(),
//     //     &pool.pubkey(),
//     // )?;
//     //
//     // // mint initial tokens to user accounts
//     // let mint_to_user_a = spl_token::instruction::mint_to(
//     //     &spl_token::ID,
//     //     &token_a_mint.pubkey(),
//     //     &user_token_a.pubkey(),
//     //     &payer.pubkey(),
//     //     &[&payer.pubkey()],
//     //     1000000, // Amount to mint
//     // )?;
//     //
//     // let mint_to_user_b = spl_token::instruction::mint_to(
//     //     &spl_token::ID,
//     //     &token_b_mint.pubkey(),
//     //     &user_token_b.pubkey(),
//     //     &payer.pubkey(),
//     //     &[&payer.pubkey()],
//     //     1000000, // Amount to mint
//     // )?;
//
//     // first transaction: handle mint creation
//     program
//         .request()
//         .instruction(create_mint_a)
//         .instruction(init_mint_a?)
//         .instruction(create_mint_b)
//         .instruction(init_mint_b?)
//         .instruction(create_lp_token)
//         .instruction(init_lp_mint?)
//         .signer(&token_a_mint)
//         .signer(&token_b_mint)
//         .signer(&lp_token_mint)
//         .send()?;
//
//     // second transaction: handle user token accounts
//     // program
//     //     .request()
//     //     .instruction(create_user_token_a)
//     //     .instruction(init_user_token_a)
//     //     .instruction(create_user_token_b)
//     //     .instruction(init_user_token_b)
//     //     .instruction(create_user_lp_token)
//     //     .instruction(init_user_lp_token)
//     //     .signer(&user_token_a)
//     //     .signer(&user_token_b)
//     //     .signer(&user_lp_token)
//     //     .send()?;
//     //
//     // // third transaction: handle pool token accounts and minting
//     // program
//     //     .request()
//     //     .instruction(create_pool_token_a)
//     //     .instruction(init_pool_token_a)
//     //     .instruction(create_pool_token_b)
//     //     .instruction(init_pool_token_b)
//     //     .instruction(mint_to_user_a)
//     //     .instruction(mint_to_user_b)
//     //     .signer(&pool_token_a)
//     //     .signer(&pool_token_b)
//     //     .send()?;
//
//     // // Before creating accounts, try to close them if they exist
//     // let close_account_ix = system_instruction::transfer(&pool.pubkey(), &payer.pubkey(), 0);
//     // program
//     //     .request()
//     //     .instruction(close_account_ix)
//     //     .signer(&pool)
//     //     .send()
//     //     .unwrap_or_default(); // Ignore error if account doesn't exist
//     // sleep(Duration::from_secs(1));
//     //
//
//     // add before pool initialization
//     let create_pool_account = system_instruction::create_account(
//         &payer.pubkey(),
//         &pool.pubkey(),
//         program
//             .rpc()
//             .get_minimum_balance_for_rent_exemption(8 + 32 + 32 + 8 + 8 + 32)?,
//         8 + 32 + 32 + 8 + 8 + 32,
//         &rusted_dex::ID,
//     );
//
//     // first initialize the pool
//     let init_tx = program
//         .request()
//         .instruction(create_pool_account)
//         .accounts(accounts::InitializePool {
//             pool: pool.pubkey(),
//             user: payer.pubkey(),
//             system_program: system_program::ID,
//         })
//         .args(instruction::InitializePool {
//             token_a: token_a_mint.pubkey(),
//             token_b: token_b_mint.pubkey(),
//         })
//         .signer(&pool)
//         .send()
//         .map_err(|e| format!("Transaction failed: {}", e))?;
//
//     println!("Pool initialized with tx: {}", init_tx);
//
//     // // now test add liquidity
//     // let add_liquidity_tx = program
//     //     .request()
//     //     .accounts(accounts::AddLiquidity {
//     //         pool: pool.pubkey(),
//     //         user: payer.pubkey(),
//     //         user_token_a: user_token_a.pubkey(),
//     //         user_token_b: user_token_b.pubkey(),
//     //         pool_token_a: pool_token_a.pubkey(),
//     //         pool_token_b: pool_token_b.pubkey(),
//     //         lp_token_mint: lp_token_mint.pubkey(),
//     //         user_lp_token: user_lp_token.pubkey(),
//     //         token_program: spl_token::ID,
//     //     })
//     //     .args(instruction::AddLiquidity {
//     //         amount_a: 200,
//     //         amount_b: 100,
//     //     })
//     //     .signer(&payer)
//     //     .send()
//     //     .map_err(|e| format!("Transaction failed: {}", e))?;
//     //
//     // println!(
//     //     "Liquidity added successfully. Signature: {}",
//     //     add_liquidity_tx
//     // );
//     //
//     // sleep(Duration::from_secs(1));
//     //
//     // // After add_liquidity_tx
//     // let user_token_a_balance = program
//     //     .rpc()
//     //     .get_token_account_balance(&user_token_a.pubkey())?;
//     // let user_token_b_balance = program
//     //     .rpc()
//     //     .get_token_account_balance(&user_token_b.pubkey())?;
//     // let user_lp_balance = program
//     //     .rpc()
//     //     .get_token_account_balance(&user_lp_token.pubkey())?;
//     //
//     // println!("Final balances:");
//     // println!("Token A: {}", user_token_a_balance.ui_amount.unwrap());
//     // println!("Token B: {}", user_token_b_balance.ui_amount.unwrap());
//     // println!("LP tokens: {}", user_lp_balance.ui_amount.unwrap());
//
//     Ok(())
// }
