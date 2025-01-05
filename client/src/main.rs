use anchor_client::{Client, Cluster};
use anchor_spl::token::{spl_token, Mint};
use clap::{Parser, Subcommand};
use rusted_dex::{accounts, instruction, SwapTokenParams};
use solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signature::Keypair,
    signer::{EncodableKey, Signer},
    system_instruction, system_program, transaction,
};
use std::{error::Error, rc::Rc};

#[derive(Parser)]
#[command(name = "rusted_dex")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    RustedDex,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // create client
    let cluster = Cluster::Devnet;
    let supplier = Rc::new(Keypair::read_from_file(
        "/Users/admin/.config/solana/id.json",
    )?);
    let payer = Rc::new(Keypair::new());
    let client = Client::new(cluster, payer.clone());

    // create program
    let program = client.program(rusted_dex::ID)?;

    let (pool_addr, _) =
        Pubkey::find_program_address(&[b"pool", payer.pubkey().as_ref()], &rusted_dex::ID);
    let token_a = Pubkey::new_unique();
    let token_b = Pubkey::new_unique();

    // mint account keypairs
    let token_a_mint = Keypair::new();
    let token_b_mint = Keypair::new();
    let lp_token_mint = Keypair::new();

    // user account keypairs
    let user_token_a = Keypair::new();
    let user_token_b = Keypair::new();
    let user_lp_token = Keypair::new();

    // pool account keypairs
    let pool_token_a = Keypair::new();
    let pool_token_b = Keypair::new();

    // rent balances
    let rpc_client = program.rpc();
    let mint_rent = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    let account_rent =
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
        account_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_user_token_b = system_instruction::create_account(
        &payer.pubkey(),
        &user_token_b.pubkey(),
        account_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_user_lp_token = system_instruction::create_account(
        &payer.pubkey(),
        &user_lp_token.pubkey(),
        account_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    // create pool accounts
    let create_pool_token_a = system_instruction::create_account(
        &payer.pubkey(),
        &pool_token_a.pubkey(),
        account_rent,
        spl_token::state::Account::LEN as u64,
        &spl_token::ID,
    );

    let create_pool_token_b = system_instruction::create_account(
        &payer.pubkey(),
        &pool_token_b.pubkey(),
        account_rent,
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

    let init_user_lp_token = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &user_lp_token.pubkey(),
        &lp_token_mint.pubkey(),
        &payer.pubkey(),
    );

    let init_pool_token_a = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &pool_token_a.pubkey(),
        &token_a_mint.pubkey(),
        &payer.pubkey(),
    );

    let init_pool_token_b = spl_token::instruction::initialize_account(
        &spl_token::ID,
        &pool_token_b.pubkey(),
        &token_b_mint.pubkey(),
        &payer.pubkey(),
    );

    // mint initial tokens to accounts
    let mint_to_user_a = spl_token::instruction::mint_to(
        &spl_token::ID,
        &token_a_mint.pubkey(),
        &user_token_a.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        10_000,
    );

    let mint_to_user_b = spl_token::instruction::mint_to(
        &spl_token::ID,
        &token_b_mint.pubkey(),
        &user_token_b.pubkey(),
        &payer.pubkey(),
        &[&payer.pubkey()],
        10_000,
    );

    match &cli.command {
        Commands::RustedDex => {
            // initialize pool
            println!("initializing the pool...");
            add_sol(&client, &supplier, &payer.pubkey())?;
            let init_tx = program
                .request()
                .accounts(accounts::InitializePool {
                    pool: pool_addr,
                    user: program.payer(),
                    system_program: system_program::ID,
                })
                .args(instruction::InitializePool { token_a, token_b })
                .signer(&payer)
                .send()
                .map_err(|e| format!("❌initialize pool failed: {}", e))?;
            println!("✅initialize pool successful");

            let tx_url = format!("https://explorer.solana.com/tx/{}?cluster=devnet", init_tx);
            println!("🔗initialize pool url: {}", tx_url);

            // add liquidity
            println!("adding liquidity to the pool...");
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

            println!("✅add liquidity successful");

            let tx_url = format!("https://explorer.solana.com/tx/{}?cluster=devnet", add_tx);
            println!("🔗add liquidity url: {}", tx_url);

            // swap token
            println!("swapping tokens...");

            let amount_in = 1000;
            let minimum_output_amount = 450; // slippage protection
            let swap_params = SwapTokenParams {
                amount_in,
                minimum_output_amount,
            };

            let swap_tx = program
                .request()
                .accounts(accounts::SwapToken {
                    pool: pool_addr,
                    user: payer.pubkey(),
                    user_input_token_account: user_token_a.pubkey(),
                    user_output_token_account: user_token_b.pubkey(),
                    pool_input_token_account: pool_token_a.pubkey(),
                    pool_output_token_account: pool_token_b.pubkey(),
                    token_program: spl_token::ID,
                })
                .args(instruction::SwapToken {
                    params: swap_params,
                })
                .signer(&payer)
                .send()
                .map_err(|e| format!("❌swap token failed: {}", e))?;

            println!("✅swap token successful");

            let tx_url = format!("https://explorer.solana.com/tx/{}?cluster=devnet", swap_tx);
            println!("🔗swap token url: {}", tx_url);
        }
    }

    Ok(())
}

fn add_sol(
    client: &Client<Rc<Keypair>>,
    sender: &Keypair,
    recipient: &Pubkey,
) -> Result<(), Box<dyn Error>> {
    let one_sol = 1_000_000_000.0;
    let amount = 30_000_000;

    println!(
        "transferring {} sol to {}...",
        amount as f64 / one_sol,
        recipient
    );

    let program = client.program(rusted_dex::ID)?;

    let transaction = {
        let blockhash = program.rpc().get_latest_blockhash()?;
        let instructions = vec![system_instruction::transfer(
            &sender.pubkey(),
            recipient,
            amount,
        )];

        transaction::Transaction::new_signed_with_payer(
            &instructions,
            Some(&sender.pubkey()),
            &[sender],
            blockhash,
        )
    };

    let signature = program.rpc().send_and_confirm_transaction(&transaction)?;

    let sender_balance = program.rpc().get_balance(&sender.pubkey())?;
    let recipient_balance = program.rpc().get_balance(recipient)?;

    println!(
        "✅transferred {} sol from {} to {}",
        amount as f64 / one_sol,
        sender.pubkey(),
        recipient
    );
    println!("sender's balance: {} sol", sender_balance as f64 / one_sol);
    println!(
        "recipient's balance: {} sol",
        recipient_balance as f64 / one_sol
    );

    let tx_url = format!(
        "https://explorer.solana.com/tx/{}?cluster=devnet",
        signature,
    );
    println!("🔗add sol url: {}", tx_url);

    Ok(())
}
